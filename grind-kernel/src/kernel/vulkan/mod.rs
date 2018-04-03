use std::sync::Arc;

use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::instance::DeviceExtensions;
use vulkano::instance::Features;
use vulkano::swapchain::Surface;

pub fn is_available() -> bool {
    match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => true,
        Err(err) => false,
    }
}

pub struct VulkanDriver {
    device: Arc<Device>,
}

impl VulkanDriver {
    pub fn from_wayland<D, S>(display: *const D, surface: *const S) -> VulkanDriver {
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
        let surface = unsafe { Surface::from_wayland(instance.clone(), display, surface, display) };

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
            let caps = window
                .capabilities(physical)
                .expect("failed to get surface capabilities");

            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            Swapchain::new(
                device.clone(),
                window.clone(),
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

        // Create renderpass
        let render_pass = Arc::new(
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
            }
        ).unwrap(),
        );

        // Create pipeline
        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        // Create framebuffers
        let mut framebuffers: Option<Vec<Arc<vulkano::framebuffer::Framebuffer<_, _>>>> = None;

        VulkanDriver { device }
    }
}
