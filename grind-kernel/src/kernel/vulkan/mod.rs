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

    pub fn set_buffer_data(&self, buffer: &mut Buffer, data: &[u8]) {
        buffer.set_data(&self.context, data);
    }

    pub fn new_shader(&self, spirv: &[u32], shader_type: vo::ShaderStageFlags) -> Shader {
        Shader::new(&self.context, spirv, shader_type)
    }

    pub fn draw(
        &mut self,
        vs: Arc<Shader>,
        fs: Arc<Shader>,
        buffers: HashMap<u32, Arc<Buffer>>,
        attrs: Arc<VertexAttributes>,
    ) {
        // PIPELINE CREATION

        // Stages
        let vertex_stage = vo::PipelineShaderStage {
            module: vs.module.clone(),
            stage: vs.as_ref().shader_stage
        };

        let fragment_stage = vo::PipelineShaderStage {
            module: fs.module.clone(),
            stage: fs.as_ref().shader_stage
        };
        let stages = vec![vertex_stage, fragment_stage];

        // Vertex input
        let vertex_input = vo::PipelineVertexInputState{
            bindings: attrs.get_vertex_input_binding_description(),
            attributes: attrs.get_vertex_input_attribute_description()
        };

        // Input assembly
        let input_assembly = vo::PipelineInputAssemblyState {topology: vo::PrimitiveTopology::TRIANGLE_LIST};

        let viewports = vec![vo::Viewport {
            x: 0.,
            y: 300.,
            width: 300.,
            height: 300.,
            min_depth: 0.,
            max_depth: 1.
        }];

        let scissors = vec![vo::Rect2D {
            offset: vo::Offset2D { x: 0, y: 0 },
            extent: vo::Extent2D { width: 300, height: 300},
        }];
        
        let viewport_state = vo::PipelineViewportState {
            viewports: viewports,
            scissors: scissors
        };

        let rasterization = vo::PipelineRasterizationState {
            polygon_mode: vo::PolygonMode::FILL,
            line_width: 1.,
            cull_mode: vo::CullModeFlags::FRONT,
            front_face: vo::FrontFace::COUNTER_CLOCKWISE,
            depth_clamp_enable: false,
            depth_bias_constant: 0.,
            depth_bias_clamp: 0.,
            depth_bias_slope: 0.
        };

        let multisample = vo::PipelineMultisampleState {
            shading_enable: false,
            samples: vo::SampleCountFlags::TYPE_1,
            min_sample_shading: 1.
        };

        // Depth stencil
        let front_stencil = vo::StencilOpState {
            fail_op: vo::StencilOp::KEEP,
            pass_op: vo::StencilOp::KEEP,
            depth_fail_op: vo::StencilOp::KEEP,
            compare_op: vo::CompareOp::EQUAL,
            compare_mask: 1,
            write_mask: 1,
            reference: 1,
        };
        let back_stencil = vo::StencilOpState {
            fail_op: vo::StencilOp::KEEP,
            pass_op: vo::StencilOp::KEEP,
            depth_fail_op: vo::StencilOp::KEEP,
            compare_op: vo::CompareOp::EQUAL,
            compare_mask: 1,
            write_mask: 1,
            reference: 1,
        };
        let depth = vo::PipelineDepthStencilState {
            depth_test_enable: false,
            depth_write_enable: false,
            depth_bounds_test_enable: false,
            depth_compare: vo::CompareOp::EQUAL,
            stencil_test_enable: false,
            front: front_stencil,
            back: back_stencil,
            min: 0.,
            max: 1.
        };

        let blend = vo::PipelineColorBlendState {
            op_enable: false,
            op: vo::LogicOp::AND,
            attachments: Vec::new(),
            constants: [0., 0., 0., 0.]
        };

        let dynamic = vo::PipelineDynamicState {
            states: Vec::new()
        };

        let layout = vo::PipelineLayout::new(&self.context, Vec::new());

        let attachment = vo::AttachmentDescription::builder()
            .format(self.context.swapchain_format)
            .samples(vo::SampleCountFlags::TYPE_1)
            .load_op(vo::AttachmentLoadOp::LOAD)
            .store_op(vo::AttachmentStoreOp::STORE)
            .build();
        let attachment_reference = vo::AttachmentReference {
            attachment: 0,
            layout: vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
        };
        let subpass = vo::SubpassDescription::builder()
            .pipeline_bind_point(vo::PipelineBindPoint::GRAPHICS)
            .color_attachments(&[attachment_reference])
            .build();
        let render_pass = vo::RenderPass::new(&self.context, vec![attachment], vec![subpass], Vec::new());

        let pipeline = vo::Pipeline::new(
            &self.context, stages, vertex_input, input_assembly,
            viewport_state, rasterization, multisample, depth,
            blend, dynamic, layout, &render_pass);

        let image_view = self.context.swapchain_image_views[self.context.current_swapchain_image as usize].clone();
        let framebuffer = vo::Framebuffer::new(&self.context, &render_pass, vec![image_view], 300, 300, 1);

        // Buffer hashmap to vec
        let mut buffers_vec = Vec::new();
        for (k, v) in buffers.iter() {
            buffers_vec.push(v.buffer.as_ref().unwrap());
        }

        vo::immediate_buffer(&self.context, |cb| {
            let render_area = vo::Rect2D {
                offset: vo::Offset2D { x: 0, y: 0},
                extent: vo::Extent2D { width: 300, height: 300}
            };

            let clear_value = vo::ClearValue {
                color: vo::ClearColorValue { float32: [1.0, 0., 0., 1.]}
            };
            cb.begin_render_pass(&self.context, &render_pass, framebuffer, render_area, vec![clear_value], vo::SubpassContents::INLINE);
            cb.bind_pipeline(&self.context, &pipeline);
            cb.bind_vertex_buffers(&self.context, 0, &buffers_vec);
            cb.draw(&self.context, 3, 1, 0, 0);
            cb.end_render_pass(&self.context);
        });

        println!("RUST ICI");
    }

    pub fn read_pixels(&mut self, x: i32, y: i32, width: i32, height: i32, desired_format: vo::Format, pixels: *mut c_void) {
        // Wait device to be ready (for GL Front Buffer)
        self.context.wait_device_idle();

        // Create buffer
        let size = width * height * 4;
        let flags = vo::BufferUsageFlags::TRANSFER_DST;
        let memory_properties = vo::MemoryPropertyFlags::HOST_VISIBLE | vo::MemoryPropertyFlags::HOST_COHERENT;
        let buffer = vo::Buffer::new(&self.context, size as vo::DeviceSize, flags, memory_properties);

        let destination_image = vo::Image::new(
            &self.context, vo::ImageType::TYPE_2D, desired_format,
            width as u32, height as u32, 1, 1, 1, vo::SampleCountFlags::TYPE_1,
            vo::SharingMode::EXCLUSIVE, vo::ImageTiling::OPTIMAL,
            vo::ImageUsageFlags::TRANSFER_SRC | vo::ImageUsageFlags::TRANSFER_DST,
            vo::MemoryPropertyFlags::DEVICE_LOCAL);

        // Put image in DST TRANSFERT LAYOUT desired layout
        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &destination_image,
                vo::ImageLayout::UNDEFINED, vo::ImageLayout::TRANSFER_DST_OPTIMAL,
                vo::PipelineStageFlags::TRANSFER, vo::PipelineStageFlags::TRANSFER,
                vo::AccessFlags::default(), vo::AccessFlags::TRANSFER_WRITE,
                0, 1);
        });

        // Copy image from source to destination
        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vo::ImageLayout::TRANSFER_SRC_OPTIMAL,
                vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vo::PipelineStageFlags::TRANSFER,
                vo::AccessFlags::COLOR_ATTACHMENT_WRITE, vo::AccessFlags::TRANSFER_READ,
                0, 1);

            if self.context.get_current_image().image_format == destination_image.image_format {
                cmd.copy_image(&self.context, &self.context.get_current_image(), &destination_image);
            }
            else {
                cmd.blit_image(&self.context, &self.context.get_current_image(), &destination_image);
            }

            cmd.update_image_layout(
                &self.context, &self.context.get_current_image(),
                vo::ImageLayout::TRANSFER_SRC_OPTIMAL, vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vo::PipelineStageFlags::TRANSFER, vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vo::AccessFlags::TRANSFER_READ, vo::AccessFlags::COLOR_ATTACHMENT_WRITE,
                0, 1);
        });

        // Copy image from destination to buffer
        vo::immediate_buffer(&self.context, |cmd| {
            cmd.update_image_layout(
                &self.context, &destination_image,
                vo::ImageLayout::TRANSFER_DST_OPTIMAL, vo::ImageLayout::TRANSFER_SRC_OPTIMAL,
                vo::PipelineStageFlags::TRANSFER, vo::PipelineStageFlags::TRANSFER,
                vo::AccessFlags::TRANSFER_WRITE, vo::AccessFlags::TRANSFER_READ,
                0, 1);

            cmd.copy_image_to_buffer(
                &self.context,
                &destination_image,
                &buffer
            );

            cmd.update_image_layout(
                &self.context, &destination_image,
                vo::ImageLayout::TRANSFER_SRC_OPTIMAL, vo::ImageLayout::TRANSFER_DST_OPTIMAL,
                vo::PipelineStageFlags::TRANSFER, vo::PipelineStageFlags::TRANSFER,
                vo::AccessFlags::TRANSFER_READ, vo::AccessFlags::TRANSFER_WRITE,
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
