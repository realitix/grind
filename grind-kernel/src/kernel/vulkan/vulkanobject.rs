use std::ptr;
use std::os::raw::{c_void};

use ash::vk;
use ash::Device;

use ash::version::{InstanceV1_0, DeviceV1_0};

use kernel::vulkan::vulkancontext::VulkanContext;

// This module is just a proxy for theses objects
pub use ash::vk::{
    MemoryPropertyFlags, 
    SubmitInfo,
    StructureType,
    Fence,
    DeviceMemory,
    DeviceSize,
    BufferUsageFlags,
    SharingMode,
    DependencyFlags,
    MemoryBarrier,
    BufferMemoryBarrier,
    ImageLayout,
    PipelineStageFlags,
    AccessFlags,
    ImageAspectFlags,
    ClearColorValue,
    ImageSubresourceRange,
    ImageSubresourceLayers,
    Offset3D,
    Extent3D,
    Filter,
    CommandBufferLevel,
    Format,
    ImageType,
    SampleCountFlags,
    ImageTiling,
    ImageUsageFlags,
    ImageCreateFlags,
    ImageViewType,
    ComponentMapping,
    ComponentSwizzle,
};


// ----------
// FUNCTIONS
// ----------
fn find_memory_type(context: &VulkanContext, type_filter: u32, properties: MemoryPropertyFlags) -> u32 {
    let cache_properties = unsafe {
        context.instance
        .get_physical_device_memory_properties(context.physical_device)
    };

    for (i, memory_type) in cache_properties.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) != 0 && (memory_type.property_flags & properties) == properties {
            return i as u32;
        }
    }
    0
}


pub fn immediate_buffer<F>(context: &VulkanContext, f: F)
    where
        F: FnOnce(&CommandBuffer)
{
    // TODO: Should be TRANSIENT
    let pool = CommandPool::new(context);
    let buffers = pool.allocate_buffers(context, 1);

    // Call the function
    buffers[0].begin(context);
    f(&buffers[0]);
    buffers[0].end(context);
    
    // Now submit the command
    let submit = SubmitInfo {
        s_type: StructureType::SUBMIT_INFO,
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
pub struct Buffer {
    buffer: vk::Buffer,
    memory: DeviceMemory,
    size: DeviceSize
}

impl Buffer {
    pub fn new(context: &VulkanContext, size: DeviceSize, usage: BufferUsageFlags, memory_properties: MemoryPropertyFlags) -> Buffer {
        // Create buffer
        let indices = [(context.queue_family_index as u32)];
        let buffer_create = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .queue_family_indices(&indices);

        let buffer = unsafe { context.device.create_buffer(&buffer_create, None).unwrap() };

        // Create memory
        let requirements = unsafe { context.device.get_buffer_memory_requirements(buffer) };
        let memory_create = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(find_memory_type(context, requirements.memory_type_bits, memory_properties));

        let memory = unsafe { context.device.allocate_memory(&memory_create, None).unwrap() };

        // Bind memory to buffer
        unsafe { context.device.bind_buffer_memory(buffer, memory, 0).unwrap() };

        Buffer {
            buffer, memory, size
        }
    }

    pub fn bind<F>(&self, context: &VulkanContext, f: F) 
    where
        F: FnOnce(*mut c_void)
    {
        let data = unsafe { context.device.map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty()).unwrap() };
        f(data);
        unsafe { context.device.unmap_memory(self.memory) };
    }
}

pub struct CommandBuffer {
    buffer: vk::CommandBuffer
}

impl CommandBuffer {
    pub fn new(buffer: vk::CommandBuffer) -> CommandBuffer {
        CommandBuffer { buffer }
    }

    pub fn begin(&self, context: &VulkanContext) {
        let cmd_create = vk::CommandBufferBeginInfo::builder();
        unsafe { context.device.begin_command_buffer(self.buffer, &cmd_create).unwrap() };
    }

    pub fn end(&self, context: &VulkanContext) {
        unsafe { context.device.end_command_buffer(self.buffer).unwrap() };
    }

    pub fn pipeline_barrier(&self, context: &VulkanContext, src_stage_mask: vk::PipelineStageFlags, 
                            dst_stage_mask: vk::PipelineStageFlags, dependency_flags: DependencyFlags,
                            memory_barriers: &[MemoryBarrier], buffer_memory_barriers: &[BufferMemoryBarrier],
                            image_memory_barriers: &[vk::ImageMemoryBarrier]) {
        unsafe {
            context.device.cmd_pipeline_barrier(
            self.buffer, src_stage_mask, dst_stage_mask,
            dependency_flags, memory_barriers, buffer_memory_barriers,
            image_memory_barriers)
        };
    }

    pub fn update_image_layout(&self, context: &VulkanContext, image: &Image, old_layout: ImageLayout,
                               new_layout: ImageLayout, src_stage: PipelineStageFlags,
                               dst_stage: PipelineStageFlags, src_access: AccessFlags,
                               dst_access: AccessFlags, base_mip_level: u32, mip_levels: u32 ) {
        let subresource_range = vk::ImageSubresourceRange {
            aspect_mask: ImageAspectFlags::COLOR,
            base_mip_level: base_mip_level,
            level_count: mip_levels,
            base_array_layer: 0,
            layer_count: 1
        };

        let barrier = vk::ImageMemoryBarrier::builder()
            .src_access_mask(src_access)
            .dst_access_mask(dst_access)
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image.image)
            .subresource_range(subresource_range)
            .build();

        self.pipeline_barrier(
            context, src_stage, dst_stage,
            DependencyFlags::empty(), &[], &[], &[barrier]);
    }

    pub fn clear_color_image(&self, context: &VulkanContext, image: &Image, layout: ImageLayout, clear_color: &ClearColorValue, ranges: &[ImageSubresourceRange]) {
        unsafe { context.device.cmd_clear_color_image(self.buffer, image.image, layout, clear_color, ranges) };
    }

    // Image must be to TransferDstOptimal layout
    pub fn copy_image_to_buffer(&self, context: &VulkanContext, image: &Image, buffer: &Buffer) {
        let image_subresource = ImageSubresourceLayers {
            aspect_mask: ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1
        };
        let image_offset = Offset3D {
            x: 0, y: 0, z: 0
        };
        let image_extent = Extent3D {
            width: image.width,
            height: image.height,
            depth: image.depth
        };
        let region = vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: image_subresource,
            image_offset: image_offset,
            image_extent: image_extent
        };

        unsafe {
            context.device.cmd_copy_image_to_buffer(
                self.buffer,
                image.image,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer.buffer,
                &[region]
            );
        }
    }

    pub fn copy_image(&self, context: &VulkanContext, src_image: &Image, dst_image: &Image) {
        let src_subresource = ImageSubresourceLayers::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build();
        let dst_subresource = ImageSubresourceLayers::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let src_offset = Offset3D {x: 0, y: 0, z: 0};
        let dst_offset = Offset3D {x: 0, y: 0, z: 0};

        let extent = Extent3D {
            width: src_image.width,
            height: src_image.height,
            depth: 1
        };

        let image_copy = vk::ImageCopy::builder()
            .src_subresource(src_subresource)
            .dst_subresource(dst_subresource)
            .src_offset(src_offset)
            .dst_offset(dst_offset)
            .extent(extent)
            .build();

        let regions = [image_copy];
        unsafe {
            context.device.cmd_copy_image(
                self.buffer,
                src_image.image,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst_image.image,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions
            );
        }
    }

    pub fn blit_image(&self, context: &VulkanContext, src_image: &Image, dst_image: &Image) {
        let src_subresource = ImageSubresourceLayers::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build();
        let dst_subresource = ImageSubresourceLayers::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let src_offset = Offset3D {x: src_image.width as i32, y: src_image.height as i32, z: 1};
        let dst_offset = Offset3D {x: dst_image.width as i32, y: dst_image.height as i32, z: 1};

        let image_blit = vk::ImageBlit::builder()
            .src_subresource(src_subresource)
            .dst_subresource(dst_subresource)
            .src_offsets([Offset3D {x: 0, y: 0, z: 0}, src_offset])
            .dst_offsets([Offset3D {x: 0, y: 0, z: 0}, dst_offset])
            .build();

        let regions = [image_blit];
        unsafe {
            context.device.cmd_blit_image(
                self.buffer,
                src_image.image,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst_image.image,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions,
                Filter::NEAREST
            );
        }
    }
}


pub struct CommandPool {
    pool: vk::CommandPool
}

impl CommandPool {
    pub fn new(context: &VulkanContext) -> CommandPool {
        let commandpool_create = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(context.queue_family_index as u32);

        let pool = unsafe { context.device.create_command_pool(&commandpool_create, None).unwrap() };

        CommandPool { pool }
    }

    pub fn allocate_buffers(&self, context: &VulkanContext, count: u32) -> Vec<CommandBuffer> {
        let commandbuffers_create = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        let buffers = unsafe { context.device.allocate_command_buffers(&commandbuffers_create).unwrap() };
        let mut new_buffers = Vec::new();
        for buffer in buffers.into_iter() {
            new_buffers.push(CommandBuffer::new(buffer));
        }

        new_buffers
    }

    pub fn free_buffers(&self, context: &VulkanContext, buffers: Vec<CommandBuffer>) {
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

pub struct Image {
    pub image : vk::Image,
    pub memory: DeviceMemory,
    pub image_format: Format,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32
}

impl Image {
    pub fn new(context: &VulkanContext, image_type: ImageType, image_format: Format,
               width: u32, height: u32, depth: u32, mip_levels: u32, layers: u32,
               samples: SampleCountFlags, sharing_mode: SharingMode, tiling: ImageTiling,
               usage: ImageUsageFlags, memory_properties: MemoryPropertyFlags)
               -> Image {

        // Check image can be created
        unsafe {
            context.instance
            .get_physical_device_image_format_properties(
                context.physical_device, image_format, image_type, tiling,
                usage, ImageCreateFlags::empty())
            .expect("Can't create your image")
        };

        // Create image
        let extent = Extent3D { width, height, depth };
        let indices = [context.queue_family_index as u32];
        let image_create = vk::ImageCreateInfo::builder()
            .image_type(image_type)
            .format(image_format)
            .extent(extent)
            .mip_levels(mip_levels)
            .array_layers(layers)
            .samples(samples)
            .tiling(tiling)
            .usage(usage)
            .sharing_mode(sharing_mode)
            .queue_family_indices(&indices)
            .initial_layout(ImageLayout::UNDEFINED);

        let image = unsafe { context.device.create_image(&image_create, None).unwrap() };

        // Create memory
        let requirements = unsafe { context.device.get_image_memory_requirements(image) };
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(find_memory_type(context, requirements.memory_type_bits, memory_properties));
        let memory = unsafe { context.device.allocate_memory(&alloc_info, None).unwrap() };

        // Bind memory to image
        unsafe { context.device.bind_image_memory(image, memory, 0).unwrap() };

        Image { image, memory: memory, image_format, width, height, depth, mip_levels }
    }

    pub fn from_swapchain_image(swapchain_image: vk::Image, width: u32,
                                height: u32, image_format: Format) -> Image {
        Image {
            image: swapchain_image,
            memory: DeviceMemory::null(),
            image_format: image_format,
            width: width,
            height: height,
            depth: 1,
            mip_levels: 1
        }           
    }

    pub fn clone(&self) -> Image {
        Image {
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


pub struct ImageView {
    pub image: Image,
    image_view: vk::ImageView
}

impl ImageView {
    pub fn new(context: &VulkanContext, image: Image, view_type: ImageViewType,
               format: Format, subresource_range: ImageSubresourceRange) -> ImageView {
        ImageView::from_device(&context.device, image, view_type, format, subresource_range)
    }

    pub fn from_device(device: &Device, image: Image, view_type: ImageViewType,
                       format: Format, subresource_range: ImageSubresourceRange) -> ImageView {
        let components = ComponentMapping {
            r: ComponentSwizzle::IDENTITY,
            g: ComponentSwizzle::IDENTITY,
            b: ComponentSwizzle::IDENTITY,
            a: ComponentSwizzle::IDENTITY
        };

        let image_view_create = vk::ImageViewCreateInfo::builder()
            .image(image.image)
            .view_type(view_type)
            .format(format)
            .components(components)
            .subresource_range(subresource_range);

        let image_view = unsafe { device.create_image_view(&image_view_create, None).unwrap() };

        ImageView { image, image_view }
    }
}


pub struct Semaphore {
    pub semaphore: vk::Semaphore
}

impl Semaphore {
    pub fn new(context: &VulkanContext) -> Semaphore {
        let semaphore_create_info = vk::SemaphoreCreateInfo::builder();
        let semaphore = unsafe { context.device.create_semaphore(&semaphore_create_info, None).unwrap() };

        Semaphore { semaphore }
    }
}


/// A shader module is a Spir-V shader loaded into Vulkan.
/// After being created, it must be inserted in a pipeline stage.
/// The real Vulkan module can be accessed by the 'module' property.
pub struct ShaderModule {
    module: vk::ShaderModule
}

pub struct Pipeline {

}

impl Pipeline {
    pub fn new(context: &VulkanContext) -> Pipeline {
        Pipeline {}
    }
}