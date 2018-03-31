mod types;
mod context;
mod display;
mod entrypoint;
mod config;

pub mod wayland;

use egl::types::*;

// ----------
// Configuration Management
// ----------
#[no_mangle]
pub extern "C" fn gk_eglGetConfigs(
    dpy: EGLDisplay,
    configs: *mut EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> EGLBoolean {
    entrypoint::get_configs(dpy, configs, config_size, num_config)
}

#[no_mangle]
pub extern "C" fn gk_eglGetConfigAttrib(
    dpy: EGLDisplay,
    config: EGLConfig,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglGetConfigAttrib");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglChooseConfig(
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
pub extern "C" fn gk_eglCreateWindowSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    win: EGLNativeWindowType,
    attrib_list: *const EGLint,
) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglCreateWindowSurface");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn gk_eglCreatePbufferSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    attrib_list: *const EGLint,
) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglCreatePBufferSurface");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn gk_eglCreatePbufferFromClientBuffer(
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
pub extern "C" fn gk_eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglDestroySurface");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglCreatePixmapSurface(
    dpy: EGLDisplay,
    config: EGLConfig,
    pixmap: EGLNativePixmapType,
    attrib_list: *const EGLint,
) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglCreatePixmapSurface");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn gk_eglSurfaceAttrib(
    dpy: EGLDisplay,
    surface: EGLSurface,
    attribute: EGLint,
    value: EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglSurfaceAttrib");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglQuerySurface(
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
pub extern "C" fn gk_eglBindAPI(api: EGLenum) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglBindAPI");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglQueryAPI() -> EGLenum {
    println!("Grind-Kernel: FN not implemented: eglQueryAPI");
    EGL_OPENGL_ES_API
}

#[no_mangle]
pub extern "C" fn gk_eglCreateContext(
    dpy: EGLDisplay,
    config: EGLConfig,
    share_context: EGLContext,
    attrib_list: *const EGLint,
) -> EGLContext {
    println!("Grind-Kernel: FN not implemented: eglCreateContext");
    EGL_NO_CONTEXT
}

#[no_mangle]
pub extern "C" fn gk_eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglDestroyContext");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglMakeCurrent(
    dpy: EGLDisplay,
    draw: EGLSurface,
    read: EGLSurface,
    ctx: EGLContext,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglMakeCurrent");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglGetCurrentContext() -> EGLContext {
    println!("Grind-Kernel: FN not implemented: eglGetCurrentContext");
    EGL_NO_CONTEXT
}

#[no_mangle]
pub extern "C" fn gk_eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface {
    println!("Grind-Kernel: FN not implemented: eglGetCurrentContext");
    EGL_NO_SURFACE
}

#[no_mangle]
pub extern "C" fn gk_eglGetCurrentDisplay() -> EGLDisplay {
    println!("Grind-Kernel: FN not implemented: eglGetCurrentDisplay");
    EGL_NO_DISPLAY
}

#[no_mangle]
pub extern "C" fn gk_eglQueryContext(
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
pub extern "C" fn gk_eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay {
    entrypoint::get_display(display_id)
}

#[no_mangle]
pub extern "C" fn gk_eglInitialize(
    dpy: EGLDisplay,
    major: *mut EGLint,
    minor: *mut EGLint,
) -> EGLBoolean {
    entrypoint::initialize(dpy, major, minor)
}

#[no_mangle]
pub extern "C" fn gk_eglTerminate(dpy: EGLDisplay) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglTerminate");
    EGL_FALSE
}

/*I don't know hot to convert to c_char right now
 * #[no_mangle]
pub extern "C" fn gk_eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char {
    println!("Grind-Kernel: FN not implemented: eglQueryString");
    "test"
}*/

#[no_mangle]
pub extern "C" fn gk_eglReleaseThread() -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglReleaseThread");
    EGL_FALSE
}

// ----------
// Synchronization Primitives
// ----------
#[no_mangle]
pub extern "C" fn gk_eglWaitClient() -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglWaitClient");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglWaitGL() -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglWaitGL");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglWaitNative(engine: EGLint) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglWaitNative");
    EGL_FALSE
}

// ----------
// Posting the Color Buffer
// ----------
#[no_mangle]
pub extern "C" fn gk_eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglSwapBuffers");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglCopyBuffers(
    dpy: EGLDisplay,
    surface: EGLSurface,
    target: EGLNativePixmapType,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglCopyBuffers");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglSwapInterval");
    EGL_FALSE
}

// ----------
// Render to Textures
// ----------
#[no_mangle]
pub extern "C" fn gk_eglBindTexImage(
    dpy: EGLDisplay,
    surface: EGLSurface,
    buffer: EGLint,
) -> EGLBoolean {
    println!("Grind-Kernel: FN not implemented: eglBindTexImage");
    EGL_FALSE
}

#[no_mangle]
pub extern "C" fn gk_eglReleaseTexImage(
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
//#[no_mangle]
//pub extern "C" fn gk_eglGetProcAddress(procname: *const c_char) -> *const () {
//    println!("Grind-Kernel: FN not implemented: eglReleaseTexImage");
//    gk_eglGetError as *const ()
//}

// ----------
// Errors
// ----------
#[no_mangle]
pub extern "C" fn gk_eglGetError() -> EGLint {
    entrypoint::get_error()
}
