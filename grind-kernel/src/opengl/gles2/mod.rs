use kernel::vulkan::VulkanDriver;
use opengl::types::*;
use std::sync::Arc;

pub struct ContextGlES2 {
    kernel: VulkanDriver,
    clear_color: [GLclampf; 4],
}

impl ContextGlES2 {
    pub fn new(kernel: VulkanDriver) -> ContextGlES2 {
        ContextGlES2 {
            kernel,
            clear_color: [0.; 4],
        }
    }

    pub fn clear_color(&mut self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        self.clear_color = [red, green, blue, alpha]
    }

    pub fn clear(&mut self, mask: GLbitfield) {
        self.kernel.clear(self.clear_color);
    }

    pub fn swap_buffers(&mut self) {
        self.kernel.present();
    }
}
