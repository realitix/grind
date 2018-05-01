use std::sync::Arc;
use std::ptr::Unique;
use libc::c_void;

use vulkano::device::Queue;
use vulkano::device::Device;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::framebuffer::RenderPass;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;
use vulkano::sync::GpuFuture;

pub struct Renderer {
    device: Arc<Device>,
    surface: Arc<Surface<Unique<c_void>>>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Unique<c_void>>>,
    //future: GpuFuture
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        surface: Arc<Surface<Unique<c_void>>>,
        queue: Arc<Queue>,
        swapchain: Arc<Swapchain<Unique<c_void>>>,
    ) -> Renderer {
        //let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap();

        Renderer {
            device,
            surface,
            queue,
            swapchain,
            //command_buffer
        }
    }
}
