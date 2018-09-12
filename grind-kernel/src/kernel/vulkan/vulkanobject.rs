use std::ptr;

use ash;
use ash::vk;
use ash::version::{InstanceV1_0, DeviceV1_0, V1_0};

use kernel::vulkan::vulkancontext::VulkanContext;
pub use ash::vk::types::*;


// -----------
// LOW LEVEL TYPES
// -----------
pub type Device = ash::Device<V1_0>;


// ----------
// FUNCTIONS
// ----------
fn find_memory_type(context: &VulkanContext, type_filter: u32, properties: MemoryPropertyFlags) -> u32 {
    let cache_properties = context.instance.get_physical_device_memory_properties(context.physical_device);
    for (i, memory_type) in cache_properties.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) != 0 && (memory_type.property_flags & properties) == properties {
            return i as u32;
        }
    }

    0
}


pub fn immediate_buffer<F>(context: &VulkanContext, f: F)
    where
        F: FnOnce(&GrindCommandBuffer)
{
    // TODO: Should be TRANSIENT
    let pool = GrindCommandPool::new(context);
    let buffers = pool.allocate_buffers(context, 1);

    // Call the function
    buffers[0].begin(context);
    f(&buffers[0]);
    buffers[0].end(context);
    
    // Now submit the command
    let submit = SubmitInfo {
        s_type: StructureType::SubmitInfo,
        p_next: ptr::null(),
        wait_semaphore_count: 0,
        p_wait_semaphores: ptr::null(),
        p_wait_dst_stage_mask: ptr::null(),
        command_buffer_count: 1,
        p_command_buffers: &buffers[0].buffer,
        signal_semaphore_count: 0,
        p_signal_semaphores: ptr::null()
    };

    unsafe {
        context.device.queue_submit(context.present_queue, &[submit], Fence::null());
        context.device.queue_wait_idle(context.present_queue);
    }

    pool.free_buffers(context, buffers);
    pool.destroy(context);
}



// ----------
// HIGH-LEVEL STRUCTS
// ----------
pub struct GrindCommandBuffer {
    buffer: CommandBuffer
}

impl GrindCommandBuffer {
    pub fn new(buffer: CommandBuffer) -> GrindCommandBuffer {
        GrindCommandBuffer { buffer }
    }

    pub fn begin(&self, context: &VulkanContext) {
        let cmd_create = CommandBufferBeginInfo {
            s_type: StructureType::CommandBufferBeginInfo,
            p_next: ptr::null(),
            flags: CommandBufferUsageFlags::empty(),
            p_inheritance_info: ptr::null()
        };

        unsafe { context.device.begin_command_buffer(self.buffer, &cmd_create).unwrap() };
    }

    pub fn end(&self, context: &VulkanContext) {
        unsafe { context.device.end_command_buffer(self.buffer).unwrap() };
    }

    pub fn pipeline_barrier(&self, context: &VulkanContext, src_stage_mask: PipelineStageFlags, 
                            dst_stage_mask: PipelineStageFlags, dependency_flags: DependencyFlags,
                            memory_barriers: &[MemoryBarrier], buffer_memory_barriers: &[BufferMemoryBarrier],
                            image_memory_barriers: &[ImageMemoryBarrier]) {
        unsafe {
            context.device.cmd_pipeline_barrier(
            self.buffer, src_stage_mask, dst_stage_mask,
            dependency_flags, memory_barriers, buffer_memory_barriers,
            image_memory_barriers)
        };
    }

    pub fn update_image_layout(&self, context: &VulkanContext, image: &GrindImage, old_layout: ImageLayout,
                               new_layout: ImageLayout, src_stage: PipelineStageFlags,
                               dst_stage: PipelineStageFlags, src_access: AccessFlags,
                               dst_access: AccessFlags, base_mip_level: u32, mip_levels: u32 ) {
        let subresource_range = ImageSubresourceRange {
            aspect_mask: IMAGE_ASPECT_COLOR_BIT,
            base_mip_level: base_mip_level,
            level_count: mip_levels,
            base_array_layer: 0,
            layer_count: 1
        };

        let barrier = ImageMemoryBarrier {
            s_type: StructureType::ImageMemoryBarrier,
            p_next: ptr::null(),
            src_access_mask: src_access,
            dst_access_mask: dst_access,
            old_layout: old_layout,
            new_layout: new_layout,
            src_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
            image: image.image,
            subresource_range: subresource_range
        };

        self.pipeline_barrier(
            context, src_stage, dst_stage,
            DependencyFlags::empty(), &[], &[], &[barrier]);
    }

    pub fn clear_color_image(&self, context: &VulkanContext, image: GrindImage, layout: ImageLayout, clear_color: &ClearColorValue, ranges: &[ImageSubresourceRange]) {
        unsafe { context.device.cmd_clear_color_image(self.buffer, image.image, layout, clear_color, ranges) };
    }
}


pub struct GrindCommandPool {
    pool: CommandPool
}

impl GrindCommandPool {
    pub fn new(context: &VulkanContext) -> GrindCommandPool {
        let commandpool_create = CommandPoolCreateInfo {
            s_type: StructureType::CommandPoolCreateInfo,
            p_next: ptr::null(),
            flags: CommandPoolCreateFlags::empty(),
            queue_family_index: context.queue_family_index as u32
        };
        let pool = unsafe { context.device.create_command_pool(&commandpool_create, None).unwrap() };

        GrindCommandPool { pool }
    }

    pub fn allocate_buffers(&self, context: &VulkanContext, count: u32) -> Vec<GrindCommandBuffer> {
        let commandbuffers_create = CommandBufferAllocateInfo {
            s_type: StructureType::CommandBufferAllocateInfo,
            p_next: ptr::null(),
            command_pool: self.pool,
            level: CommandBufferLevel::Primary,
            command_buffer_count: count
        };

        let buffers = unsafe { context.device.allocate_command_buffers(&commandbuffers_create).unwrap() };
        let mut new_buffers = Vec::new();
        for buffer in buffers.into_iter() {
            new_buffers.push(GrindCommandBuffer::new(buffer));
        }

        new_buffers
    }

    pub fn free_buffers(&self, context: &VulkanContext, buffers: Vec<GrindCommandBuffer>) {
        let mut vk_buffers = Vec::new();

        for buffer in buffers.into_iter() {
            vk_buffers.push(buffer.buffer);
        }

        unsafe { context.device.free_command_buffers(self.pool, &vk_buffers) };
    }

    pub fn destroy(&self, context: &VulkanContext) {
        unsafe { context.device.destroy_command_pool(self.pool, None) };
    }

}

pub struct GrindImage {
    pub image : Image,
    memory: DeviceMemory,
    image_format: Format,
    width: u32,
    height: u32,
    depth: u32,
    mip_levels: u32
}

impl GrindImage {
    pub fn new(context: &VulkanContext, image_type: ImageType, image_format: Format,
               width: u32, height: u32, depth: u32, mip_levels: u32, layers: u32,
               samples: SampleCountFlags, sharing_mode: SharingMode, tiling: ImageTiling,
               usage: ImageUsageFlags, layout: ImageLayout, queue_families: Vec<u32>,
               memory_properties: MemoryPropertyFlags) -> GrindImage {

        // Check image can be created
        context.instance.get_physical_device_image_format_properties(
            context.physical_device, image_format, image_type, tiling,
            usage, ImageCreateFlags::empty())
            .expect("Can't create your image");

        // Create image
        let extent = Extent3D { width, height, depth };
        let image_create = ImageCreateInfo {
            s_type: StructureType::ImageCreateInfo,
            p_next: ptr::null(),
            flags: ImageCreateFlags::empty(),
            image_type: image_type,
            format: image_format,
            extent: extent,
            mip_levels: mip_levels,
            array_layers: layers,
            samples: samples,
            tiling: tiling,
            usage: usage,
            sharing_mode: sharing_mode,
            queue_family_index_count: queue_families.len() as u32,
            p_queue_family_indices: queue_families.as_ptr() as *const u32,
            initial_layout: layout
        };

        let image = unsafe { context.device.create_image(&image_create, None).unwrap() };

        // Create memory
        let requirements = context.device.get_image_memory_requirements(image);
        let alloc_info = MemoryAllocateInfo {
            s_type: StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: requirements.size,
            memory_type_index: find_memory_type(context, requirements.memory_type_bits, memory_properties)
        };
        let memory = unsafe { context.device.allocate_memory(&alloc_info, None).unwrap() };

        // Bind memory to image
        unsafe { context.device.bind_image_memory(image, memory, 0).unwrap() };

        GrindImage { image, memory: memory, image_format, width, height, depth, mip_levels }
    }

    pub fn from_swapchain_image(swapchain_image: vk::types::Image, width: u32,
                                height: u32, image_format: Format) -> GrindImage {
        GrindImage {
            image: swapchain_image,
            memory: vk::types::DeviceMemory::null(),
            image_format: image_format,
            width: width,
            height: height,
            depth: 1,
            mip_levels: 1
        }           
    }

    pub fn clone(&self) -> GrindImage {
        GrindImage {
            image : self.image,
            memory: self.memory,
            image_format: self.image_format,
            width: self.width,
            height: self.height,
            depth: self.depth,
            mip_levels: self.mip_levels
        }
    }
}


pub struct GrindImageView {
    pub image: GrindImage,
    image_view: ImageView
}

impl GrindImageView {
    pub fn new(context: &VulkanContext, image: GrindImage, view_type: ImageViewType,
               format: Format, subresource_range: ImageSubresourceRange) -> GrindImageView {
        GrindImageView::from_device(&context.device, image, view_type, format, subresource_range)
    }

    pub fn from_device(device: &Device, image: GrindImage, view_type: ImageViewType,
                       format: Format, subresource_range: ImageSubresourceRange) -> GrindImageView {
        let components = ComponentMapping {
            r: ComponentSwizzle::Identity,
            g: ComponentSwizzle::Identity,
            b: ComponentSwizzle::Identity,
            a: ComponentSwizzle::Identity
        };

        let image_view_create = ImageViewCreateInfo {
            s_type: StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            flags: ImageViewCreateFlags::empty(),
            image: image.image,
            view_type: view_type,
            format: format,
            components: components,
            subresource_range: subresource_range
        };

        let image_view = unsafe { device.create_image_view(&image_view_create, None).unwrap() };

        GrindImageView { image, image_view }
    }
}