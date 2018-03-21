use kernel::Kernel;
use kernel::vulkan::VulkanDriver;
use kernel::vulkan::is_available as vulkan_is_available;

use egl::wayland::WaylandDisplay;

pub fn is_available() -> bool {
    vulkan_is_available()
}

pub struct Display {
    native_display: WaylandDisplay,
    kernel: VulkanDriver,
}

impl Display {
    pub fn new(native_display: WaylandDisplay, kernel: VulkanDriver) -> Display {
        Display {
            native_display,
            kernel,
        }
    }
}
