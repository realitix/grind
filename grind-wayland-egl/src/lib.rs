extern crate libc;

use libc::{c_int, c_void};

// ----------
// GLOBAL TYPES
// ----------
type WlEglWindow = *const c_void;
type WlSurface = *const c_void;

#[link(name = "grindkernel")]
extern "C" {
    fn gk_wl_egl_window_create(surface: WlSurface, width: c_int, height: c_int) -> WlEglWindow;
    fn gk_wl_egl_window_destroy(egl_window: WlEglWindow);
    fn gk_wl_egl_window_resize(
        egl_window: WlEglWindow,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn gk_wl_egl_window_get_attached_size(
        egl_window: WlEglWindow,
        width: *mut c_int,
        height: *mut c_int,
    );
}

// ----------
// EXPORTED FUNCTIONS
// ----------

#[no_mangle]
pub fn wl_egl_window_create(surface: WlSurface, width: c_int, height: c_int) -> WlEglWindow {
    unsafe { gk_wl_egl_window_create(surface, width, height) }
}

#[no_mangle]
pub fn wl_egl_window_destroy(egl_window: WlEglWindow) {
    unsafe { gk_wl_egl_window_destroy(egl_window) }
}

#[no_mangle]
pub fn wl_egl_window_resize(
    egl_window: WlEglWindow,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    unsafe { gk_wl_egl_window_resize(egl_window, width, height, dx, dy) }
}

#[no_mangle]
pub fn wl_egl_window_get_attached_size(
    egl_window: WlEglWindow,
    width: *mut c_int,
    height: *mut c_int,
) {
    unsafe { gk_wl_egl_window_get_attached_size(egl_window, width, height) }
}
