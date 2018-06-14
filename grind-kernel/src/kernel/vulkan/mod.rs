pub mod buffer;
mod renderer;
pub mod shader;

use libc::c_void;
use std::collections::HashMap;
use std::ptr::Unique;
use std::sync::Arc;

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

use kernel::vulkan::buffer::Buffer;
use kernel::vulkan::buffer::VertexAttributes;
use kernel::vulkan::renderer::Renderer;
use kernel::vulkan::shader::Shader;

pub fn is_available() -> bool {
    match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => true,
        Err(err) => false,
    }
}

pub struct VulkanDriver {
    renderer: Renderer,
    callback: DebugCallback,
}

impl VulkanDriver {
    pub fn from_wayland(display: *mut c_void, surface: *mut c_void) -> VulkanDriver {
        // Instance
        let ideal = InstanceExtensions {
            khr_surface: true,
            khr_wayland_surface: true,
            ext_debug_report: true,
            ..InstanceExtensions::none()
        };

        let extensions = {
            match InstanceExtensions::supported_by_core() {
                Ok(supported) => supported.intersection(&ideal),
                Err(_) => InstanceExtensions::none(),
            }
        };

        // DEBUG with layer
        let layer = "VK_LAYER_LUNARG_standard_validation";
        let layers = vec![&layer];
        let instance = Instance::new(None, &extensions, layers).expect("Can't create Instance");

        let mut mt = MessageTypes::none();
        mt.information = false;
        mt.debug = false;
        mt.warning = true;
        mt.error = true;
        let callback = DebugCallback::new(&instance, mt, |msg| {
            println!("Debug callback: {:?}", msg.description);
        }).ok()
            .unwrap();

        // Physical device
        let physical_device = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no device available");

        // Surface
        let surface = unsafe {
            Surface::from_wayland(
                instance.clone(),
                display,
                surface,
                Unique::new(display).unwrap(),
            )
        }.expect("Can't create surface");

        // Queue family
        let queue_family = {
            let mut qr = None;
            for qf in physical_device.queue_families() {
                if surface.is_supported(qf).unwrap() {
                    qr = Some(qf);
                }
            }
            if qr.is_none() {
                panic!("No queue family available");
            } else {
                qr.unwrap()
            }
        };

        // Logical device
        let (device, mut queues) = {
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            };

            match Device::new(
                physical_device,
                physical_device.supported_features(),
                &device_ext,
                Some((queue_family, 1.0)),
            ) {
                Ok(d) => d,
                Err(err) => panic!("Couldn't build device: {:?}", err),
            }
        };

        // Get queue
        let queue = queues.next().unwrap();

        // Create swapchain
        let (mut swapchain, mut images) = {
            let caps = surface
                .capabilities(physical_device)
                .expect("failed to get surface capabilities");

            //let alpha = caps.supported_composite_alpha.iter().next().unwrap();

            // The format must be a Unorm one
            fn get_best_format(caps: &Capabilities) -> Format {
                for (format, color_space) in caps.supported_formats.iter() {
                    if *format == Format::B8G8R8A8Unorm
                        || *format == Format::R8G8B8A8Unorm
                            && *color_space == ColorSpace::SrgbNonLinear
                    {
                        return *format;
                    }
                }
                panic!("No acceptable format");
            }
            let format = get_best_format(&caps);

            let dimensions = [300, 300];
            let num_images = caps.min_image_count;
            Swapchain::new(
                device.clone(),
                surface.clone(),
                num_images,
                format,
                dimensions,
                1,
                caps.supported_usage_flags,
                SharingMode::Exclusive(0),
                SurfaceTransform::Identity,
                CompositeAlpha::Opaque,
                PresentMode::Fifo,
                true,
                None,
            ).expect("failed to create swapchain")
        };

        VulkanDriver {
            callback,
            renderer: Renderer::new(device, surface, queue, swapchain, images),
        }
    }

    pub fn clear(&mut self, colors: [f32; 4]) {
        self.renderer.clear(colors);
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }

    pub fn new_buffer(&self) -> Buffer {
        Buffer::new(self.renderer.get_device())
    }

    pub fn new_shader(&self, spirv: &[u8], shader_type: GraphicsShaderType) -> Shader {
        Shader::new(self.renderer.get_device(), spirv, shader_type)
    }

    pub fn draw(
        &mut self,
        vs: Arc<Shader>,
        fs: Arc<Shader>,
        buffers: HashMap<u32, Arc<Buffer>>,
        attrs: Arc<VertexAttributes>,
    ) {
        self.renderer.draw(vs, fs, buffers, attrs);
    }
}
