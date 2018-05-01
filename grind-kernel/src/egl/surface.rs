use libc::c_void;
use std::ptr::Unique;
use std::sync::Arc;

use egl::config::Config;
use egl::display::Display;
use egl::types::EGLNativeWindowType;

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
        VulkanDriver::from_wayland(
            self.clone_display_id().as_ptr(),
            (*self.clone_win()).as_ptr(),
        )
    }
}

// LocalSurface to be stored in a LocalContext
// This is mandatory to avoid multithreading on LocalContext
pub struct LocalSurface {
    kernel: Arc<VulkanDriver>,
}

impl LocalSurface {
    pub fn new(global_surface: &GlobalSurface) -> LocalSurface {
        let kernel = Arc::new(VulkanDriver::from_wayland(
            global_surface.clone_display_id().as_ptr(),
            (*global_surface.clone_win()).as_ptr(),
        ));
        LocalSurface { kernel }
    }

    pub fn clone_kernel(&self) -> Arc<VulkanDriver> {
        self.kernel.clone()
    }

    pub fn swap_buffers(&mut self) {
        Arc::get_mut(&mut self.kernel).unwrap().present();
    }
}
