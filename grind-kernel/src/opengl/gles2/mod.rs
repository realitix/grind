use std::sync::Arc;
use kernel::vulkan::VulkanDriver;
use opengl::types::*;

pub struct ContextGlES2 {
    kernel: Arc<VulkanDriver>,
    clear_color: [GLclampf; 4],
}

impl ContextGlES2 {
    pub fn new(kernel: Arc<VulkanDriver>) -> ContextGlES2 {
        ContextGlES2 {
            kernel,
            clear_color: [0.; 4],
        }
    }

    pub fn clear_color(&mut self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        self.clear_color = [red, green, blue, alpha]
    }

    pub fn clear(&mut self, mask: GLbitfield) {}
}
