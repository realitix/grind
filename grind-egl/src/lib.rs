#![allow(non_snake_case)]

extern crate khronos;
extern crate libc;

use khronos::khronos_int32_t;
use libc::{c_char, c_uint, c_void};

// ----------
// GLOBAL TYPES
// ----------
pub type EGLDisplay = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = c_uint;
pub type EGLNativeWindowType = *mut c_void;
pub type EGLSurface = *mut c_void;
pub type EGLenum = c_uint;
pub type EGLClientBuffer = *mut c_void;
pub type EGLNativePixmapType = *mut c_void;
pub type EGLContext = *mut c_void;
pub type EGLNativeDisplayType = *mut c_void;

#[link(name = "grindkernel")]
extern "C" {
    fn gk_eglGetConfigs(
        dpy: EGLDisplay,
        configs: EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EGLBoolean;
    fn gk_eglGetConfigAttrib(
        dpy: EGLDisplay,
        config: EGLConfig,
        attribute: EGLint,
        value: *mut EGLint,
    ) -> EGLBoolean;
    fn gk_eglChooseConfig(
        dpy: EGLDisplay,
        attrib_list: *const EGLint,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EGLBoolean;
    fn gk_eglCreateWindowSurface(
        dpy: EGLDisplay,
        config: EGLConfig,
        win: EGLNativeWindowType,
        attrib_list: *const EGLint,
    ) -> EGLSurface;
    fn gk_eglCreatePbufferSurface(
        dpy: EGLDisplay,
        config: EGLConfig,
        attrib_list: *const EGLint,
    ) -> EGLSurface;
    fn gk_eglCreatePbufferFromClientBuffer(
        dpy: EGLDisplay,
        buftype: EGLenum,
        buffer: EGLClientBuffer,
        config: EGLConfig,
        attrib_list: *const EGLint,
    ) -> EGLSurface;
    fn gk_eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn gk_eglCreatePixmapSurface(
        dpy: EGLDisplay,
        config: EGLConfig,
        pixmap: EGLNativePixmapType,
        attrib_list: *const EGLint,
    ) -> EGLSurface;
    fn gk_eglSurfaceAttrib(
        dpy: EGLDisplay,
        surface: EGLSurface,
        attribute: EGLint,
        value: EGLint,
    ) -> EGLBoolean;
    fn gk_eglQuerySurface(
        dpy: EGLDisplay,
        surface: EGLSurface,
        attribute: EGLint,
        value: *mut EGLint,
    ) -> EGLBoolean;
    fn gk_eglBindAPI(api: EGLenum) -> EGLBoolean;
    fn gk_eglQueryAPI() -> EGLenum;
    fn gk_eglCreateContext(
        dpy: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext;
    fn gk_eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    fn gk_eglMakeCurrent(
        dpy: EGLDisplay,
        draw: EGLSurface,
        read: EGLSurface,
        ctx: EGLContext,
    ) -> EGLBoolean;
    fn gk_eglGetCurrentContext() -> EGLContext;
    fn gk_eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;
    fn gk_eglGetCurrentDisplay() -> EGLDisplay;
    fn gk_eglQueryContext(
        dpy: EGLDisplay,
        ctx: EGLContext,
        attribute: EGLint,
        value: *mut EGLint,
    ) -> EGLBoolean;
    fn gk_eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;
    fn gk_eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;
    fn gk_eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
    //fn gk_eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char;
    fn gk_eglReleaseThread() -> EGLBoolean;
    fn gk_eglWaitClient() -> EGLBoolean;
    fn gk_eglWaitGL() -> EGLBoolean;
    fn gk_eglWaitNative(engine: EGLint) -> EGLBoolean;
    fn gk_eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn gk_eglCopyBuffers(
        dpy: EGLDisplay,
        surface: EGLSurface,
        target: EGLNativePixmapType,
    ) -> EGLBoolean;
    fn gk_eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;
    fn gk_eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    fn gk_eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    fn gk_eglGetProcAddress(procname: *const c_char) -> extern "C" fn();
    fn gk_eglGetError() -> EGLint;
}

// ----------
// EXPORTED FUNCTIONS
// ----------

#[no_mangle]
pub extern "C" fn eglGetConfigs(
    dpy: EGLDisplay,
    configs: EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> EGLBoolean {
    unsafe { gk_eglGetConfigs(dpy, configs, config_size, num_config) }
}

#[no_mangle]
pub extern "C" fn eglGetConfigAttrib(
    dpy: EGLDisplay,
    config: EGLConfig,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    unsafe { gk_eglGetConfigAttrib(dpy, config, attribute, value) }
}

#[no_mangle]
pub extern "C" fn eglChooseConfig(
    dpy: EGLDisplay,
    attrib_list: *const EGLint,
    configs: *mut EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> EGLBoolean {
    unsafe { gk_eglChooseConfig(dpy, attrib_list, configs, config_size, num_config) }
}

// ----------
// Rendering Surfaces
// ----------
#[no_mangle]
pub extern "C" fn eglCreateWindowSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    win: EGLNativeWindowType,
    attrib_list: *const EGLint,
) -> EGLSurface {
    unsafe { gk_eglCreateWindowSurface(dpy, config, win, attrib_list) }
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    attrib_list: *const EGLint,
) -> EGLSurface {
    unsafe { gk_eglCreatePbufferSurface(dpy, config, attrib_list) }
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferFromClientBuffer(
    dpy: EGLDisplay,
    buftype: EGLenum,
    buffer: EGLClientBuffer,
    config: EGLConfig,
    attrib_list: *const EGLint,
) -> EGLSurface {
    unsafe { gk_eglCreatePbufferFromClientBuffer(dpy, buftype, buffer, config, attrib_list) }
}

#[no_mangle]
pub extern "C" fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe { gk_eglDestroySurface(dpy, surface) }
}

#[no_mangle]
pub extern "C" fn eglCreatePixmapSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    pixmap: EGLNativePixmapType,
    attrib_list: *const EGLint,
) -> EGLSurface {
    unsafe { gk_eglCreatePixmapSurface(dpy, config, pixmap, attrib_list) }
}

#[no_mangle]
pub extern "C" fn eglSurfaceAttrib(
    dpy: EGLDisplay,
    surface: EGLSurface,
    attribute: EGLint,
    value: EGLint,
) -> EGLBoolean {
    unsafe { gk_eglSurfaceAttrib(dpy, surface, attribute, value) }
}

#[no_mangle]
pub extern "C" fn eglQuerySurface(
    dpy: EGLDisplay,
    surface: EGLSurface,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    unsafe { gk_eglQuerySurface(dpy, surface, attribute, value) }
}

// ----------
// Rendering Contexts
// ----------
#[no_mangle]
pub extern "C" fn eglBindAPI(api: EGLenum) -> EGLBoolean {
    unsafe { gk_eglBindAPI(api) }
}

#[no_mangle]
pub extern "C" fn eglQueryAPI() -> EGLenum {
    unsafe { gk_eglQueryAPI() }
}

#[no_mangle]
pub extern "C" fn eglCreateContext(
    dpy: EGLDisplay,
    config: EGLConfig,
    share_context: EGLContext,
    attrib_list: *const EGLint,
) -> EGLContext {
    unsafe { gk_eglCreateContext(dpy, config, share_context, attrib_list) }
}

#[no_mangle]
pub extern "C" fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean {
    unsafe { gk_eglDestroyContext(dpy, ctx) }
}

#[no_mangle]
pub extern "C" fn eglMakeCurrent(
    dpy: EGLDisplay,
    draw: EGLSurface,
    read: EGLSurface,
    ctx: EGLContext,
) -> EGLBoolean {
    unsafe { gk_eglMakeCurrent(dpy, draw, read, ctx) }
}

#[no_mangle]
pub extern "C" fn eglGetCurrentContext() -> EGLContext {
    unsafe { gk_eglGetCurrentContext() }
}

#[no_mangle]
pub extern "C" fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface {
    unsafe { gk_eglGetCurrentSurface(readdraw) }
}

#[no_mangle]
pub extern "C" fn eglGetCurrentDisplay() -> EGLDisplay {
    unsafe { gk_eglGetCurrentDisplay() }
}

#[no_mangle]
pub extern "C" fn eglQueryContext(
    dpy: EGLDisplay,
    ctx: EGLContext,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    unsafe { gk_eglQueryContext(dpy, ctx, attribute, value) }
}

// ----------
// Initialization and Terminating
// ----------
#[no_mangle]
pub extern "C" fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay {
    unsafe { gk_eglGetDisplay(display_id) }
}

#[no_mangle]
pub extern "C" fn eglInitialize(
    dpy: EGLDisplay,
    major: *mut EGLint,
    minor: *mut EGLint,
) -> EGLBoolean {
    unsafe { gk_eglInitialize(dpy, major, minor) }
}

#[no_mangle]
pub extern "C" fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean {
    unsafe { gk_eglTerminate(dpy) }
}

/*#[no_mangle]
pub extern "C" fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char {
    unsafe { gk_eglQueryString(dpy, name) }
}*/

#[no_mangle]
pub extern "C" fn eglReleaseThread() -> EGLBoolean {
    unsafe { gk_eglReleaseThread() }
}

// ----------
// Synchronization Primitives
// ----------
#[no_mangle]
pub extern "C" fn eglWaitClient() -> EGLBoolean {
    unsafe { gk_eglWaitClient() }
}

#[no_mangle]
pub extern "C" fn eglWaitGL() -> EGLBoolean {
    unsafe { gk_eglWaitGL() }
}

#[no_mangle]
pub extern "C" fn eglWaitNative(engine: EGLint) -> EGLBoolean {
    unsafe { gk_eglWaitNative(engine) }
}

// ----------
// Posting the Color Buffer
// ----------
#[no_mangle]
pub extern "C" fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe { gk_eglSwapBuffers(dpy, surface) }
}

#[no_mangle]
pub extern "C" fn eglCopyBuffers(
    dpy: EGLDisplay,
    surface: EGLSurface,
    target: EGLNativePixmapType,
) -> EGLBoolean {
    unsafe { gk_eglCopyBuffers(dpy, surface, target) }
}

#[no_mangle]
pub extern "C" fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean {
    unsafe { gk_eglSwapInterval(dpy, interval) }
}

// ----------
// Render to Textures
// ----------
#[no_mangle]
pub extern "C" fn eglBindTexImage(
    dpy: EGLDisplay,
    surface: EGLSurface,
    buffer: EGLint,
) -> EGLBoolean {
    unsafe { gk_eglBindTexImage(dpy, surface, buffer) }
}

#[no_mangle]
pub extern "C" fn eglReleaseTexImage(
    dpy: EGLDisplay,
    surface: EGLSurface,
    buffer: EGLint,
) -> EGLBoolean {
    unsafe { gk_eglReleaseTexImage(dpy, surface, buffer) }
}

// ----------
// Obtain Extension Function Pointers
// ----------
#[no_mangle]
pub extern "C" fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn() {
    unsafe { gk_eglGetProcAddress(procname) }
}

// ----------
// Errors
// ----------
#[no_mangle]
pub extern "C" fn eglGetError() -> EGLint {
    unsafe { gk_eglGetError() }
}
