use std::ptr;
use std::ffi::CStr;
use std::os::raw::{c_void};
use std::sync::Arc;

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
    ShaderStageFlags,
    VertexInputBindingDescription,
    VertexInputAttributeDescription,
    PrimitiveTopology,
    Viewport,
    Rect2D,
    PolygonMode,
    CullModeFlags,
    FrontFace,
    CompareOp,
    StencilOpState,
    LogicOp,
    PipelineColorBlendAttachmentState,
    DescriptorSetLayoutBinding,
    DynamicState,
    AttachmentDescription,
    SubpassDescription,
    SubpassDependency,
    VertexInputRate,
    Offset2D,
    Extent2D,
    StencilOp,
    AttachmentLoadOp,
    AttachmentStoreOp,
    PipelineBindPoint,
    AttachmentReference,
    ClearValue,
    SubpassContents
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
        F: FnOnce(&CommandBufferRegister)
{
    // TODO: Should be TRANSIENT
    let pool = CommandPool::new(context);
    let buffers = pool.allocate_buffers(context, 1);

    // Call the function
    buffers[0].register(context, f);
    
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
        context.device.queue_submit(context.present_queue, &[submit], Fence::null()).unwrap();
        context.device.queue_wait_idle(context.present_queue).unwrap();
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
    size: DeviceSize,
    alive: bool
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
            buffer, memory, size, alive: true
        }
    }

    pub fn destroy(&mut self, context: &VulkanContext) {
        unsafe {
            context.device.destroy_buffer(self.buffer, None)
        };
        self.alive = false;
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

    pub fn register<F>(&self, context: &VulkanContext, f: F)
    where
        F: FnOnce(&CommandBufferRegister) {
        self.begin(context);
        let cbr = CommandBufferRegister::new(context, &self);
        f(&cbr);
        self.end(context);
    }

    pub fn begin(&self, context: &VulkanContext) {
        let cmd_create = vk::CommandBufferBeginInfo::builder();
        unsafe { context.device.begin_command_buffer(self.buffer, &cmd_create).unwrap() };
    }

    pub fn end(&self, context: &VulkanContext) {
        unsafe { context.device.end_command_buffer(self.buffer).unwrap() };
    }

}

pub struct CommandBufferRegister<'a> {
    command_buffer: &'a CommandBuffer,
    context: &'a VulkanContext
}

impl<'a> CommandBufferRegister<'a> {
    pub fn new(context: &'a VulkanContext, command_buffer: &'a CommandBuffer) -> CommandBufferRegister<'a> {
        CommandBufferRegister {
            context, command_buffer
        }
    }

    pub fn begin_render_pass(&self, render_pass: &RenderPass, framebuffer: Framebuffer, render_area: Rect2D, clears: Vec<ClearValue>, subpass_contents: SubpassContents) {
        let create = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass.renderpass)
            .framebuffer(framebuffer.framebuffer)
            .render_area(render_area)
            .clear_values(&clears)
            .build();
        
        unsafe {
            self.context.device.cmd_begin_render_pass(self.command_buffer.buffer, &create, subpass_contents)
        };
    }

    pub fn end_render_pass(&self) {
        unsafe { self.context.device.cmd_end_render_pass(self.command_buffer.buffer) };
    }

    pub fn bind_pipeline(&self, pipeline: &Pipeline) {
        unsafe {
            self.context.device.cmd_bind_pipeline(self.command_buffer.buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline)
        };
    }

    pub fn bind_vertex_buffers(&self, first_binding: u32, buffers: &Vec<&Buffer>) {
        let mut vk_buffers = Vec::new();
        for b in buffers {
            vk_buffers.push(b.buffer);
        }

        let mut vk_offsets = Vec::new();
        for _ in 0..buffers.len() {
            vk_offsets.push(0);
        }

        unsafe {
            self.context.device.cmd_bind_vertex_buffers(self.command_buffer.buffer, first_binding, &vk_buffers, &vk_offsets);
        };
    }

    pub fn draw(&self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        unsafe {
            self.context.device.cmd_draw(self.command_buffer.buffer, vertex_count, instance_count, first_vertex, first_instance);
        };
    }

    pub fn pipeline_barrier(&self, src_stage_mask: vk::PipelineStageFlags, 
                            dst_stage_mask: vk::PipelineStageFlags, dependency_flags: DependencyFlags,
                            memory_barriers: &[MemoryBarrier], buffer_memory_barriers: &[BufferMemoryBarrier],
                            image_memory_barriers: &[vk::ImageMemoryBarrier]) {
        unsafe {
            self.context.device.cmd_pipeline_barrier(
                self.command_buffer.buffer, src_stage_mask, dst_stage_mask,
                dependency_flags, memory_barriers, buffer_memory_barriers,
                image_memory_barriers)
        };
    }

    pub fn update_image_layout(&self, image: &Image, old_layout: ImageLayout,
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
            src_stage, dst_stage,
            DependencyFlags::empty(), &[], &[], &[barrier]);
    }

    pub fn clear_color_image(&self, image: &Image, layout: ImageLayout, clear_color: &ClearColorValue, ranges: &[ImageSubresourceRange]) {
        unsafe { self.context.device.cmd_clear_color_image(self.command_buffer.buffer, image.image, layout, clear_color, ranges) };
    }

    // Image must be to TransferDstOptimal layout
    pub fn copy_image_to_buffer(&self, image: &Image, buffer: &Buffer) {
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
            self.context.device.cmd_copy_image_to_buffer(
                self.command_buffer.buffer,
                image.image,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer.buffer,
                &[region]
            );
        }
    }

    pub fn copy_image(&self, src_image: &Image, dst_image: &Image) {
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
            self.context.device.cmd_copy_image(
                self.command_buffer.buffer,
                src_image.image,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst_image.image,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions
            );
        }
    }

    pub fn blit_image(&self, src_image: &Image, dst_image: &Image) {
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
            self.context.device.cmd_blit_image(
                self.command_buffer.buffer,
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

pub struct Framebuffer {
    pub framebuffer: vk::Framebuffer
}

impl Framebuffer {
    pub fn new(context: &VulkanContext, render_pass: &RenderPass, attachments: Vec<Arc<ImageView>>, width: u32, height: u32, layers: u32) -> Framebuffer{
        let mut vk_image_views = Vec::new();
        for attachment in attachments {
            vk_image_views.push(attachment.image_view);
        }

        let create = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass.renderpass)
            .attachments(&vk_image_views)
            .width(width)
            .height(height)
            .layers(layers)
            .build();

        let framebuffer = unsafe { context.device.create_framebuffer(&create, None).unwrap() };

        Framebuffer { framebuffer }
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
    module: vk::ShaderModule,
}

impl ShaderModule {
    pub fn new(context: &VulkanContext, code: &[u32]) -> ShaderModule {
        let shader_create = vk::ShaderModuleCreateInfo::builder()
            .code(code)
            .build();

        let module = unsafe { context.device.create_shader_module(&shader_create, None).unwrap() };

        ShaderModule {module}
    }
}


/// A descriptor set layout object is defined by an array of zero or more
/// descriptor bindings. Each individual descriptor binding is specified by /// a descriptor type, a count (array size) of the number of descriptors in
/// the binding, a set of shader stages that can access the binding,
/// and (if using immutable samplers) an array of sampler descriptors.
pub struct DescriptorSetLayout {
    layout: vk::DescriptorSetLayout
}

impl DescriptorSetLayout {
    pub fn new(context: &VulkanContext, bindings: Vec<DescriptorSetLayoutBinding>) -> DescriptorSetLayout{
        let create = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings).build();
        let layout = unsafe { context.device.create_descriptor_set_layout(&create, None).unwrap() };

        DescriptorSetLayout { layout }
    }
}

pub struct RenderPass {
    pub renderpass: vk::RenderPass
}

impl RenderPass {
    pub fn new(context: &VulkanContext, attachments: Vec<AttachmentDescription>, subpasses: Vec<SubpassDescription>, dependencies: Vec<SubpassDependency>) -> RenderPass {
        let create = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies)
            .build();

        let renderpass = unsafe { context.device.create_render_pass(&create, None).unwrap() };

        RenderPass { renderpass }
    }
}

/// Access to descriptor sets from a pipeline is accomplished through a
/// pipeline layout. Zero or more descriptor set layouts and zero or more
/// push constant ranges are combined to form a pipeline layout object which
/// describes the complete set of resources that can be accessed by a 
/// pipeline. The pipeline layout represents a sequence of descriptorsets
/// with each having a specific layout. This sequence of layouts isused to 
/// determine the interface between shader stages and shader resources. 
/// Each pipeline is created using a pipeline layout.
pub struct PipelineLayout {
    layout: vk::PipelineLayout
}

impl PipelineLayout {
    pub fn new(context: &VulkanContext, descriptors: Vec<DescriptorSetLayout>) -> PipelineLayout {
        let mut vk_descriptor_set_layouts = Vec::new();

        for descriptor in descriptors {
            vk_descriptor_set_layouts.push(descriptor.layout);
        }

        let create = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(&vk_descriptor_set_layouts).build();

        let layout = unsafe { context.device.create_pipeline_layout(&create, None).unwrap() };

        PipelineLayout { layout }
    }
}

pub struct PipelineShaderStage {
    pub module: Arc<ShaderModule>,
    pub stage: ShaderStageFlags
}

pub struct PipelineVertexInputState {
    pub bindings: Vec<VertexInputBindingDescription>,
    pub attributes: Vec<VertexInputAttributeDescription>
}

pub struct PipelineInputAssemblyState {
    pub topology: PrimitiveTopology
}

pub struct PipelineViewportState {
    pub viewports: Vec<Viewport>,
    pub scissors: Vec<Rect2D>
}

pub struct PipelineRasterizationState {
    pub polygon_mode: PolygonMode,
    pub line_width: f32,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_clamp_enable: bool,
    pub depth_bias_constant: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope: f32
}

pub struct PipelineMultisampleState {
    pub shading_enable: bool,
    pub samples: SampleCountFlags,
    pub min_sample_shading: f32
}

pub struct PipelineDepthStencilState {
    pub depth_test_enable: bool,
    pub depth_write_enable: bool,
    pub depth_bounds_test_enable: bool,
    pub depth_compare: CompareOp,
    pub stencil_test_enable: bool,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min: f32,
    pub max: f32
}

pub struct PipelineColorBlendState {
    pub op_enable: bool,
    pub op: LogicOp,
    pub attachments: Vec<PipelineColorBlendAttachmentState>,
    pub constants: [f32; 4]
}

pub struct PipelineDynamicState {
    pub states: Vec<DynamicState>
}

pub struct Pipeline {
    pipeline: vk::Pipeline,
    layout: PipelineLayout
}

impl Pipeline {
    pub fn new(context: &VulkanContext, stages: Vec<PipelineShaderStage>, vertex_input: PipelineVertexInputState, input_assembly: PipelineInputAssemblyState, viewport_state: PipelineViewportState, rasterization: PipelineRasterizationState, multisample: PipelineMultisampleState, depth: PipelineDepthStencilState, blend: PipelineColorBlendState, dynamic: PipelineDynamicState, layout: PipelineLayout, render_pass: &RenderPass) -> Pipeline {
        let create_stages = {
            let mut r = Vec::new();
            let main_str = CStr::from_bytes_with_nul(b"main\0").unwrap();
            for stage in stages {
                let create_stage = vk::PipelineShaderStageCreateInfo::builder()
                    .stage(stage.stage)
                    .module(stage.module.module)
                    .name(&main_str)
                    .build();
                r.push(create_stage);
            }
            r
        };
        
        let create_vertex_input = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&vertex_input.bindings)
            .vertex_attribute_descriptions(&vertex_input.attributes)
            .build();

        let create_input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(input_assembly.topology)
            .primitive_restart_enable(false)
            .build();

        let create_viewport = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewport_state.viewports)
            .scissors(&viewport_state.scissors);

        let dbe = {
            if rasterization.depth_bias_constant != 0. || rasterization.depth_bias_clamp != 0. || rasterization.depth_bias_slope != 0. {
                true
            }
            else {
                false
            }
        };

        let create_rasterization = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(rasterization.depth_clamp_enable)
            .rasterizer_discard_enable(false)
            .polygon_mode(rasterization.polygon_mode)
            .line_width(rasterization.line_width)
            .cull_mode(rasterization.cull_mode)
            .front_face(rasterization.front_face)
            .depth_bias_constant_factor(rasterization.depth_bias_constant)
            .depth_bias_clamp(rasterization.depth_bias_clamp)
            .depth_bias_slope_factor(rasterization.depth_bias_slope)
            .depth_bias_enable(dbe)
            .build();

        let create_multisample = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(multisample.shading_enable)
            .rasterization_samples(multisample.samples)
            .min_sample_shading(multisample.min_sample_shading)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false)
            .build();

        let create_depth = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(depth.depth_test_enable)
            .depth_write_enable(depth.depth_test_enable)
            .depth_compare_op(depth.depth_compare)
            .depth_bounds_test_enable(depth.depth_bounds_test_enable)
            .stencil_test_enable(depth.stencil_test_enable)
            .front(depth.front)
            .back(depth.back)
            .min_depth_bounds(depth.min)
            .max_depth_bounds(depth.max)
            .build();

        let create_blend = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(blend.op_enable)
            .logic_op(blend.op)
            .attachments(&blend.attachments)
            .blend_constants(blend.constants)
            .build();

        let create_pipeline = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&create_stages)
            .vertex_input_state(&create_vertex_input)
            .input_assembly_state(&create_input_assembly)
            .viewport_state(&create_viewport)
            .rasterization_state(&create_rasterization)
            .multisample_state(&create_multisample)
            .depth_stencil_state(&create_depth)
            .color_blend_state(&create_blend)
            .layout(layout.layout)
            .render_pass(render_pass.renderpass)
            .build();
        
        let create_pipelines = [create_pipeline];
        let pipeline = unsafe { context.device.create_graphics_pipelines(vk::PipelineCache::null(), &create_pipelines, None).unwrap()[0] };

        Pipeline { pipeline, layout }
    }
}