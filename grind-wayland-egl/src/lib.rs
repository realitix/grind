extern crate libc;

use libc::{c_void, c_int};

// ----------
// GLOBAL TYPES
// ----------
type EglWindow = *const c_void;
type Surface = *const c_void;


#[link(name = "grindkernel")]
extern "C" {
    fn gk_wl_egl_window_create(surface: Surface, width: c_int, height: c_int) -> EglWindow;
    fn gk_wl_egl_window_destroy(egl_window: EglWindow);
    fn gk_wl_egl_window_resize(egl_window: EglWindow, width: c_int, height: c_int, dx: c_int, dy: c_int);
    fn gk_wl_egl_window_get_attached_size(egl_window: EglWindow, width: *mut c_int, height: *mut c_int);
}

// ----------
// EXPORTED FUNCTIONS
// ----------

#[no_mangle]
pub fn wl_egl_window_create(surface: Surface, width: c_int, height: c_int) -> EglWindow {
    unsafe { gk_wl_egl_window_create(surface, width, height) }
}

#[no_mangle]
pub fn wl_egl_window_destroy(egl_window: EglWindow) {
    unsafe { gk_wl_egl_window_destroy(egl_window) }
}

#[no_mangle]
pub fn wl_egl_window_resize(egl_window: EglWindow, width: c_int, height: c_int, dx: c_int, dy: c_int) {
    unsafe { gk_wl_egl_window_resize(egl_window, width, height, dx, dy) }
}

#[no_mangle]
pub fn wl_egl_window_get_attached_size(egl_window: EglWindow, width: *mut c_int, height: *mut c_int) {
    unsafe { gk_wl_egl_window_get_attached_size(egl_window, width, height) }
}
