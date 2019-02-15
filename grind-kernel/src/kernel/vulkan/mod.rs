pub mod buffer;
mod vulkancontext;
mod renderer;
pub mod shader;
pub mod vulkanobject;

use std::os::raw::c_void;
use std::collections::HashMap;
use std::ptr::Unique;
use std::sync::Arc;
use std::slice;


use vulkano::instance::{layers_list, Instance, InstanceExtensions};
use vulkano::pipeline::shader::GraphicsShaderType;


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
        VulkanDriver {
            context: VulkanContext::new(display, wl_egl_window.surface, wl_egl_window.width, wl_egl_window.height)
        }
    }    

    pub fn clear(&mut self, colors: [f32; 4]) {
        let clear_color = vo::ClearColorValue {
            float32: colors
        };

        let ranges = vo::ImageSubresourceRange {
            aspect_mask: vo::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1
        };

        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vo::ImageLayout::TRANSFER_DST_OPTIMAL,
                vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vo::PipelineStageFlags::TRANSFER,
                vo::AccessFlags::COLOR_ATTACHMENT_WRITE, vo::AccessFlags::TRANSFER_WRITE,
                0, 1);

            cmd.clear_color_image(
                &self.context,
                &self.context.get_current_image(),
                vo::ImageLayout::TRANSFER_DST_OPTIMAL,
                &clear_color, 
                &[ranges]);

            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::TRANSFER_DST_OPTIMAL, vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vo::PipelineStageFlags::TRANSFER, vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vo::AccessFlags::TRANSFER_WRITE, vo::AccessFlags::COLOR_ATTACHMENT_WRITE,
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

    pub fn read_pixels(&mut self, x: i32, y: i32, width: i32, height: i32, format: vo::Format, pixels: *mut c_void) {
        // Wait device to be ready (for GL Front Buffer)
        self.context.wait_device_idle();

        // Create buffer
        let size = width * height * 4;
        let flags = vo::BufferUsageFlags::TRANSFER_DST;
        let memory_properties = vo::MemoryPropertyFlags::HOST_VISIBLE | vo::MemoryPropertyFlags::HOST_COHERENT;
        let buffer = vo::GrindBuffer::new(&self.context, size as vo::DeviceSize, flags, memory_properties);

        // Copy image in buffer
        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vo::ImageLayout::TRANSFER_SRC_OPTIMAL,
                vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vo::PipelineStageFlags::TRANSFER,
                vo::AccessFlags::COLOR_ATTACHMENT_WRITE, vo::AccessFlags::TRANSFER_READ,
                0, 1);

            cmd.copy_image_to_buffer(
                &self.context,
                &self.context.get_current_image(),
                &buffer
            );

            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::TRANSFER_SRC_OPTIMAL, vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vo::PipelineStageFlags::TRANSFER, vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vo::AccessFlags::TRANSFER_READ, vo::AccessFlags::COLOR_ATTACHMENT_WRITE,
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
