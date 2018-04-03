use egl::display::Display;
use egl::config::Config;
use egl::types::EGLNativeWindowType;

use kernel::vulkan::VulkanDriver;


pub struct Surface {
    kernel: VulkanDriver
}


impl Surface {
    pub fn new(display: &Display, config: &Config, win: EGLNativeWindowType) -> Surface {
        let kernel = VulkanDriver::from_wayland(display.native_display.display_id.as_ptr(), win);
        Surface { kernel }
    }
}
