mod renderer;

use std::sync::Arc;
use std::ptr::Unique;
use libc::c_void;

use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::instance::DeviceExtensions;
use vulkano::instance::Features;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::framebuffer::Subpass;
use vulkano::framebuffer::Framebuffer;

use kernel::vulkan::renderer::Renderer;

pub fn is_available() -> bool {
    match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => true,
        Err(err) => false,
    }
}

pub struct VulkanDriver {
    renderer: Renderer,
}

impl VulkanDriver {
    pub fn from_wayland(display: *mut c_void, surface: *mut c_void) -> VulkanDriver {
        // Instance
        let ideal = InstanceExtensions {
            khr_surface: true,
            khr_wayland_surface: true,
            ..InstanceExtensions::none()
        };

        let extensions = {
            match InstanceExtensions::supported_by_core() {
                Ok(supported) => supported.intersection(&ideal),
                Err(_) => InstanceExtensions::none(),
            }
        };

        let instance = Instance::new(None, &extensions, None).unwrap();

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
        }.unwrap();

        // Logical device
        let (device, mut queues) = {
            let queue_family = physical_device.queue_families().next().unwrap();
            let features = Features::none();
            let ext = DeviceExtensions::none();

            match Device::new(physical_device, &features, &ext, Some((queue_family, 1.0))) {
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

            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions = [100, 100];
            Swapchain::new(
                device.clone(),
                surface.clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                caps.supported_usage_flags,
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                true,
                None,
            ).expect("failed to create swapchain")
        };

        let renderer = Renderer {
            device,
            surface,
            queue,
            swapchain,
        };

        // Create renderpass
        /*let render_pass = Arc::new(
            single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }).unwrap(),
        );*/

        // Create pipeline
        /*let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer()
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );*/

        // Create framebuffers
        //let mut framebuffers: Option<Vec<Arc<Framebuffer<_, _>>>> = None;

        VulkanDriver { renderer }
    }
}
