use std::os::raw::{c_int, c_void};
use std::boxed::Box;
use std::mem;
use std::ptr;
use std::ptr::Unique;
use std::sync::Arc;

use egl::surface::SurfaceManager;
use kernel::vulkan::VulkanDriver;

// ----------
// GLOBAL TYPES
// ----------
type WlDisplay = *mut c_void;
type WlSurface = *mut c_void;

// ----------
// STRUCTS
// ----------
pub struct WlEglWindow {
    pub surface: WlSurface,
    pub width: u32,
    pub height: u32,
}

// ----------
// EXPORTED FUNCTIONS
// ----------
#[no_mangle]
pub fn gk_wl_egl_window_create(
    surface: WlSurface,
    width: c_int,
    height: c_int,
) -> *const WlEglWindow {
    // On Heap!
    let w = Box::new(WlEglWindow {
        surface: surface,
        width: width as u32,
        height: height as u32,
    });
    let pointer = Box::into_raw(w) as *const WlEglWindow;
    pointer
}

#[no_mangle]
pub fn gk_wl_egl_window_destroy(mut egl_window_ptr: *mut c_void) {
    unsafe {
        ptr::drop_in_place(egl_window_ptr as *mut WlEglWindow);
    }
}

#[no_mangle]
pub fn gk_wl_egl_window_resize(
    mut egl_window_ptr: *mut c_void,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let egl_window = unsafe { &mut *(egl_window_ptr as *mut WlEglWindow) };
    egl_window.height = height as u32;
    egl_window.width = width as u32;
}

#[no_mangle]
pub fn gk_wl_egl_window_get_attached_size(
    egl_window_ptr: *const WlEglWindow,
    width: *mut c_int,
    height: *mut c_int,
) {
    unsafe {
        let egl_window = &(*egl_window_ptr);
        *width = egl_window.width as i32;
        *height = egl_window.height as i32;
    }
}

// ----------
// Display
// ----------
pub struct WaylandDisplay {
    pub display_id: Arc<Unique<c_void>>,
}

impl WaylandDisplay {
    pub fn new(display_id: WlDisplay) -> WaylandDisplay {
        WaylandDisplay {
            display_id: Arc::new(
                Unique::new(display_id).expect("You must pass a valid pointer for wayland display"),
            ),
        }
    }
}

// ----------
// Creator
// ----------
pub struct WaylandSurfaceManager {
    display_id: Arc<Unique<c_void>>,
    win: Arc<Unique<c_void>>,
}

impl WaylandSurfaceManager {
    pub fn new(display_id: Arc<Unique<c_void>>, win: Arc<Unique<c_void>>) -> WaylandSurfaceManager {
        WaylandSurfaceManager { display_id, win }
    }
}

impl SurfaceManager for WaylandSurfaceManager {
    fn generate_kernel(&self) -> VulkanDriver {
        let c_pointer = unsafe { (*self.win).as_ref() as *const c_void as *const WlEglWindow };
        let wl_egl_window = unsafe { &(*c_pointer) };
        VulkanDriver::from_wayland(self.display_id.as_ptr(), wl_egl_window)
    }
}
