use std::ptr::Unique;
use libc::{c_int, c_void};

// ----------
// GLOBAL TYPES
// ----------
type WlDisplay = *mut c_void;
type WlEglWindow = *mut c_void;
type WlSurface = *mut c_void;

// ----------
// EXPORTED FUNCTIONS
// ----------
#[no_mangle]
pub fn gk_wl_egl_window_create(surface: WlSurface, width: c_int, height: c_int) -> WlEglWindow {
    surface as WlEglWindow
}

#[no_mangle]
pub fn gk_wl_egl_window_destroy(egl_window: WlEglWindow) {}

#[no_mangle]
pub fn gk_wl_egl_window_resize(
    egl_window: WlEglWindow,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
}

#[no_mangle]
pub fn gk_wl_egl_window_get_attached_size(
    egl_window: WlEglWindow,
    width: *mut c_int,
    height: *mut c_int,
) {
}

// ----------
// Display
// ----------
pub struct WaylandDisplay {
    display_id: Unique<c_void>,
}

impl WaylandDisplay {
    pub fn new(display_id: WlDisplay) -> WaylandDisplay {
        WaylandDisplay {
            display_id: Unique::new(display_id).unwrap(),
        }
    }
}
