use std::sync::Arc;

use egl::display::Display;
use egl::config::Config;
use egl::types::EGLNativeWindowType;

use kernel::vulkan::VulkanDriver;

pub struct Surface {
    kernel: Arc<VulkanDriver>,
}

impl PartialEq for Surface {
    fn eq(&self, other: &Surface) -> bool {
        self as *const Surface == other as *const Surface
    }
}

impl Surface {
    pub fn new(display: &Display, config: &Config, win: EGLNativeWindowType) -> Surface {
        let kernel = Arc::new(VulkanDriver::from_wayland(
            display.native_display.display_id.as_ptr(),
            win,
        ));
        Surface { kernel }
    }

    pub fn clone_kernel(&self) -> Arc<VulkanDriver> {
        self.kernel.clone()
    }

    pub fn swap_buffers(&self) {}
}
