use libc::c_void;
use std::ptr;
use std::ptr::Unique;
use std::sync::Arc;

use egl::config::Config;
use egl::display::Display;
use egl::types::EGLNativeWindowType;
use egl::wayland::WlEglWindow;

use kernel::vulkan::VulkanDriver;

pub trait SurfaceCreator: Send + Sync {
    fn generate_kernel(&self) -> VulkanDriver;
}

// GlobalSurface to be stored in a Display
pub struct GlobalSurface {
    creator: Box<SurfaceCreator>,
}

impl PartialEq for GlobalSurface {
    fn eq(&self, other: &GlobalSurface) -> bool {
        self as *const GlobalSurface == other as *const GlobalSurface
    }
}

impl GlobalSurface {
    pub fn new(creator: Box<SurfaceCreator>) -> GlobalSurface {
        GlobalSurface { creator }
    }

    pub fn create_kernel(&self) -> VulkanDriver {
        self.creator.generate_kernel()
    }
}
