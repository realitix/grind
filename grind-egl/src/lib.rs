extern crate khronos;
extern crate libc;

use khronos::khronos_int32_t;
use libc::{c_uint, c_void, int32_t, c_char};

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
extern {
    fn gk_eglGetConfigs(dpy: EGLDisplay, configs: EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;
    fn gk_eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
    fn gk_eglTest() -> i32;
}

// ----------
// EXPORTED FUNCTIONS
// ----------
#[no_mangle]
pub extern "C" fn eglTest() -> i32 {
    unsafe { gk_eglTest() }
}

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
/*
#[no_mangle]
pub extern "C" fn eglChooseConfig(
    dpy: EGLDisplay,
    attrib_list: *const EGLint,
    configs: *mut EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglChooseConfig");
    EGL_FALSE
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
    println!("Grind-Kernel: FN not implemented: eglCreateWindowSurface");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    attrib_list: *const EGLint,
) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglCreatePBufferSurface");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferFromClientBuffer(
    dpy: EGLDisplay,
    buftype: EGLenum,
    buffer: EGLClientBuffer,
    config: EGLConfig,
    attrib_list: *const EGLint,
) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglCreatePbufferFromClientBuffer");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglDestroySurface");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglCreatePixmapSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    pixmap: EGLNativePixmapType,
    attrib_list: *const EGLint,
) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglCreatePixmapSurface");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn eglSurfaceAttrib(
    dpy: EGLDisplay,
    surface: EGLSurface,
    attribute: EGLint,
    value: EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglSurfaceAttrib");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglQuerySurface(
    dpy: EGLDisplay,
    surface: EGLSurface,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglQuerySurface");
    EGL_FALSE
}


// ----------
// Rendering Contexts
// ----------
#[no_mangle]
pub extern "C" fn eglBindAPI(api: EGLenum) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglBindAPI");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglQueryAPI() -> EGLenum {
    println!("Grind-Kernel: FN not implemented: eglQueryAPI");
    EGL_OPENGL_ES_API
}

#[no_mangle]
pub extern "C" fn eglCreateContext(
    dpy: EGLDisplay,
    config: EGLConfig,
    share_context: EGLContext,
    attrib_list: *const EGLint,
) -> EGLContext {
    println!("Grind-Kernel: FN not implemented: eglCreateContext");
    EGL_NO_CONTEXT
}

#[no_mangle]
pub extern "C" fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglDestroyContext");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglMakeCurrent(
    dpy: EGLDisplay,
    draw: EGLSurface,
    read: EGLSurface,
    ctx: EGLContext,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglMakeCurrent");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglGetCurrentContext() -> EGLContext {
    println!("Grind-Kernel: FN not implemented: eglGetCurrentContext");
    EGL_NO_CONTEXT
}

#[no_mangle]
pub extern "C" fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglGetCurrentContext");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn eglGetCurrentDisplay() -> EGLDisplay {
    println!("Grind-Kernel: FN not implemented: eglGetCurrentDisplay");
    EGL_NO_DISPLAY
}

#[no_mangle]
pub extern "C" fn eglQueryContext(
    dpy: EGLDisplay,
    ctx: EGLContext,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglQueryContext");
    EGL_FALSE
}


// ----------
// Initialization and Terminating
// ----------
#[no_mangle]
pub extern "C" fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay {
    println!("Grind-Kernel: FN not implemented: eglGetDisplay");
    EGL_NO_DISPLAY
}

#[no_mangle]
pub extern "C" fn eglInitialize(
    dpy: EGLDisplay,
    major: *mut EGLint,
    minor: *mut EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglInitialize");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglTerminate");
    EGL_FALSE
}

/*I don't know hot to convert to c_char right now
 * #[no_mangle]
pub extern "C" fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char {
    println!("Grind-Kernel: FN not implemented: eglQueryString");
    "test"
}*/

#[no_mangle]
pub extern "C" fn eglReleaseThread() -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglReleaseThread");
    EGL_FALSE
}


// ----------
// Synchronization Primitives
// ----------
#[no_mangle]
pub extern "C" fn eglWaitClient() -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglWaitClient");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglWaitGL() -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglWaitGL");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglWaitNative(engine: EGLint) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglWaitNative");
    EGL_FALSE
}


// ----------
// Posting the Color Buffer
// ----------
#[no_mangle]
pub extern "C" fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglSwapBuffers");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglCopyBuffers(
    dpy: EGLDisplay,
    surface: EGLSurface,
    target: EGLNativePixmapType,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglCopyBuffers");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglSwapInterval");
    EGL_FALSE
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
    println!("Grind-Kernel: FN not implemented: eglBindTexImage");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn eglReleaseTexImage(
    dpy: EGLDisplay,
    surface: EGLSurface,
    buffer: EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglReleaseTexImage");
    EGL_FALSE
}


// ----------
// Obtain Extension Function Pointers
// ----------
#[no_mangle]
pub extern "C" fn eglGetProcAddress(procname: *const c_char) -> *const() {
    println!("Grind-Kernel: FN not implemented: eglReleaseTexImage");
    eglGetError  as *const()
}


// ----------
// Errors
// ----------
#[no_mangle]
pub extern "C" fn eglGetError() -> EGLint {
    println!("Grind-Kernel: FN not implemented: eglGetError");
    EGL_DONT_CARE
}
*/
