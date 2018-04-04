use std::sync::Arc;
use std::ptr::Unique;
use libc::c_void;

use vulkano::device::Queue;
use vulkano::device::Device;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::framebuffer::RenderPass;


pub struct Renderer {
    pub device: Arc<Device>,
    pub surface: Arc<Surface<Unique<c_void>>>,
    pub queue: Arc<Queue>,
    pub swapchain: Arc<Swapchain<Unique<c_void>>>
}


