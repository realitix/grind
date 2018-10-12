pub mod buffer;
mod vulkancontext;
mod renderer;
pub mod shader;
mod vulkanobject;

use libc::c_void;
use std::collections::HashMap;
use std::ptr::Unique;
use std::sync::Arc;
use std::slice;

use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::instance::DeviceExtensions;
use vulkano::instance::Features;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::{layers_list, Instance, InstanceExtensions};
use vulkano::pipeline::shader::GraphicsShaderType;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::Capabilities;
use vulkano::swapchain::ColorSpace;
use vulkano::swapchain::CompositeAlpha;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::sync::SharingMode;

use egl::platform::wayland::WlEglWindow;
use kernel::vulkan::buffer::Buffer;
use kernel::vulkan::buffer::VertexAttributes;
use kernel::vulkan::renderer::Renderer;
use kernel::vulkan::shader::Shader;
use kernel::vulkan::vulkancontext::VulkanContext;
use kernel::vulkan::vulkanobject as vo;

pub fn is_available() -> bool {
    match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => true,
        Err(err) => false,
    }
}

pub struct VulkanDriver {
    context: VulkanContext
}

impl VulkanDriver {
    // Create driver from wayland
    pub fn from_wayland(display: *mut c_void, wl_egl_window: &WlEglWindow) -> VulkanDriver {
        VulkanDriver{
            context: VulkanContext::new(display, wl_egl_window.surface, wl_egl_window.width, wl_egl_window.height)
        }
    }    

    pub fn clear(&mut self, colors: [f32; 4]) {
        let clear_color = vo::ClearColorValue {
            float32: colors
        };

        let ranges = vo::ImageSubresourceRange {
            aspect_mask: vo::IMAGE_ASPECT_COLOR_BIT,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1
        };

        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::ColorAttachmentOptimal, vo::ImageLayout::TransferDstOptimal,
                vo::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, vo::PIPELINE_STAGE_TRANSFER_BIT,
                vo::ACCESS_COLOR_ATTACHMENT_WRITE_BIT, vo::ACCESS_TRANSFER_WRITE_BIT,
                0, 1);

            cmd.clear_color_image(
                &self.context,
                &self.context.get_current_image(),
                vo::ImageLayout::TransferDstOptimal,
                &clear_color, 
                &[ranges]);

            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::TransferDstOptimal, vo::ImageLayout::ColorAttachmentOptimal,
                vo::PIPELINE_STAGE_TRANSFER_BIT, vo::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                vo::ACCESS_TRANSFER_WRITE_BIT, vo::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                0, 1);
        });
    }

    pub fn present(&mut self) {
        self.context.present(&[]);
        self.context.acquire();
    }

    pub fn new_buffer(&self) -> Buffer {
        Buffer::new()
    }

    pub fn new_shader(&self, spirv: &[u8], shader_type: GraphicsShaderType) -> Shader {
        Shader::new(spirv, shader_type)
    }

    pub fn draw(
        &mut self,
        vs: Arc<Shader>,
        fs: Arc<Shader>,
        buffers: HashMap<u32, Arc<Buffer>>,
        attrs: Arc<VertexAttributes>,
    ) {
        //self.renderer.draw(vs, fs, buffers, attrs);
    }

    pub fn read_pixels(&mut self, x: i32, y: i32, width: i32, height: i32, pixels: *mut c_void) {
        // Wait device to be ready (for GL Front Buffer)
        self.context.wait_device_idle();

        // Create buffer
        let size = width * height * 4;
        let flags = vo::BUFFER_USAGE_TRANSFER_DST_BIT;
        let memory_properties = vo::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vo::MEMORY_PROPERTY_HOST_COHERENT_BIT;
        let buffer = vo::GrindBuffer::new(&self.context, size as vo::DeviceSize, flags, memory_properties);

        // Copy image in buffer
        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::ColorAttachmentOptimal, vo::ImageLayout::TransferSrcOptimal,
                vo::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, vo::PIPELINE_STAGE_TRANSFER_BIT,
                vo::ACCESS_COLOR_ATTACHMENT_WRITE_BIT, vo::ACCESS_TRANSFER_READ_BIT,
                0, 1);

            cmd.copy_image_to_buffer(
                &self.context,
                &self.context.get_current_image(),
                &buffer
            );

            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::TransferSrcOptimal, vo::ImageLayout::ColorAttachmentOptimal,
                vo::PIPELINE_STAGE_TRANSFER_BIT, vo::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                vo::ACCESS_TRANSFER_READ_BIT, vo::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                0, 1);
        });

        // Bind buffer and retrieve pointer to memory
        buffer.bind(&self.context, |data| {
            unsafe {
                let pixels_slice = slice::from_raw_parts_mut(pixels as *mut u8, size as usize);
                let data_slice = slice::from_raw_parts_mut(data as *mut u8, size as usize);
                pixels_slice.copy_from_slice(data_slice);
            }
        });
    }
}
