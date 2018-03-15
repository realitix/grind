use kernel::Kernel;
use kernel::vulkan::VulkanDriver;
use kernel::vulkan::is_available as vulkan_is_available;

pub fn is_available() -> bool {
    vulkan_is_available()
}

pub struct Display {
    kernel: VulkanDriver,
}

impl Display {
    pub fn new() -> Display {
        Display {
            kernel: VulkanDriver::new(),
        }
    }
}
