use libc::c_void;
use std::ptr;
use std::ptr::Unique;
use std::sync::Arc;

use egl::config::Config;
use egl::display::Display;
use egl::types::EGLNativeWindowType;
use egl::wayland::WlEglWindow;

use kernel::vulkan::VulkanDriver;

// GlobalSurface to be stored in a Display
pub struct GlobalSurface {
    display_id: Arc<Unique<c_void>>,
    win: Arc<Unique<c_void>>,
}

impl PartialEq for GlobalSurface {
    fn eq(&self, other: &GlobalSurface) -> bool {
        self as *const GlobalSurface == other as *const GlobalSurface
    }
}

impl GlobalSurface {
    pub fn new(display_id: Arc<Unique<c_void>>, win: Arc<Unique<c_void>>) -> GlobalSurface {
        GlobalSurface { display_id, win }
    }

    pub fn clone_display_id(&self) -> Arc<Unique<c_void>> {
        Arc::clone(&self.display_id)
    }

    pub fn clone_win(&self) -> Arc<Unique<c_void>> {
        Arc::clone(&self.win)
    }

    pub fn create_kernel(&self) -> VulkanDriver {
        // Wayland is specific, we need to retrieve the WlEglWindow from self.win
        let c_pointer = unsafe { (*self.win).as_ref() as *const c_void as *const WlEglWindow };
        let wl_egl_window = unsafe { &(*c_pointer) };
        VulkanDriver::from_wayland(self.clone_display_id().as_ptr(), wl_egl_window)
    }
}
