use std::ptr::Unique;
use libc::{c_int, c_void};

use egl::types::EGLNativeDisplayType;

// ----------
// GLOBAL TYPES
// ----------
type EglWindow = *const c_void;
type Surface = *const c_void;

// ----------
// EXPORTED FUNCTIONS
// ----------
#[no_mangle]
pub fn gk_wl_egl_window_create(surface: Surface, width: c_int, height: c_int) -> EglWindow {
    surface as EglWindow
}

#[no_mangle]
pub fn gk_wl_egl_window_destroy(egl_window: EglWindow) {}

#[no_mangle]
pub fn gk_wl_egl_window_resize(
    egl_window: EglWindow,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
}

#[no_mangle]
pub fn gk_wl_egl_window_get_attached_size(
    egl_window: EglWindow,
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
    pub fn new(display_id: EGLNativeDisplayType) -> WaylandDisplay {
        WaylandDisplay {
            display_id: Unique::new(display_id).unwrap(),
        }
    }
}
