// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

extern crate khronos;
extern crate libc;

// rust
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

// system
use khronos::khronos_int32_t;

use libc::{ c_uint, c_void, int32_t, c_char };

// -------------------------------------------------------------------------------------------------
// GLOBAL TYPES
// -------------------------------------------------------------------------------------------------

pub type EGLBoolean           = c_uint;
pub type EGLClientBuffer      = *mut c_void;
pub type EGLConfig            = *mut c_void;
pub type EGLContext           = *mut c_void;
pub type EGLDisplay           = *mut c_void;
pub type EGLenum              = c_uint;
pub type EGLint               = khronos_int32_t;
pub type EGLNativeDisplayType = *mut c_void;
pub type EGLSurface           = *mut c_void;

// -------------------------------------------------------------------------------------------------
// ANDROID TYPES
// -------------------------------------------------------------------------------------------------

#[repr(C)]
#[cfg(android)]
struct android_native_window_t;

#[repr(C)]
#[cfg(android)]
struct egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativePixmapType = *mut egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativeWindowType = *mut android_native_window_t;

// -------------------------------------------------------------------------------------------------
// NON-ANDROID TYPES
// -------------------------------------------------------------------------------------------------

#[cfg(not(android))]
pub type EGLNativePixmapType = *mut c_void;

#[cfg(not(android))]
pub type EGLNativeWindowType = *mut c_void;

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct EGLConfigList {
    pub configs: EGLConfig,
    pub count:   int32_t
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

// EGL aliases
pub const EGL_FALSE: EGLBoolean = 0;
pub const EGL_TRUE:  EGLBoolean = 1;

// out-of-band handle values
pub const EGL_DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as *mut c_void;
pub const EGL_NO_CONTEXT:      EGLContext = 0 as *mut c_void;
pub const EGL_NO_DISPLAY:      EGLDisplay = 0 as *mut c_void;
pub const EGL_NO_SURFACE:      EGLSurface = 0 as *mut c_void;

// out-of-band attribute value
pub const EGL_DONT_CARE: EGLint = -1;

// errors / GetError return values
pub const EGL_SUCCESS:             EGLint = 0x3000;
pub const EGL_NOT_INITIALIZED:     EGLint = 0x3001;
pub const EGL_BAD_ACCESS:          EGLint = 0x3002;
pub const EGL_BAD_ALLOC:           EGLint = 0x3003;
pub const EGL_BAD_ATTRIBUTE:       EGLint = 0x3004;
pub const EGL_BAD_CONFIG:          EGLint = 0x3005;
pub const EGL_BAD_CONTEXT:         EGLint = 0x3006;
pub const EGL_BAD_CURRENT_SURFACE: EGLint = 0x3007;
pub const EGL_BAD_DISPLAY:         EGLint = 0x3008;
pub const EGL_BAD_MATCH:           EGLint = 0x3009;
pub const EGL_BAD_NATIVE_PIXMAP:   EGLint = 0x300A;
pub const EGL_BAD_NATIVE_WINDOW:   EGLint = 0x300B;
pub const EGL_BAD_PARAMETER:       EGLint = 0x300C;
pub const EGL_BAD_SURFACE:         EGLint = 0x300D;
pub const EGL_CONTEXT_LOST:        EGLint = 0x300E;  // EGL 1.1 - IMG_power_management

// config attributes
pub const EGL_BUFFER_SIZE:             EGLint = 0x3020;
pub const EGL_ALPHA_SIZE:              EGLint = 0x3021;
pub const EGL_BLUE_SIZE:               EGLint = 0x3022;
pub const EGL_GREEN_SIZE:              EGLint = 0x3023;
pub const EGL_RED_SIZE:                EGLint = 0x3024;
pub const EGL_DEPTH_SIZE:              EGLint = 0x3025;
pub const EGL_STENCIL_SIZE:            EGLint = 0x3026;
pub const EGL_CONFIG_CAVEAT:           EGLint = 0x3027;
pub const EGL_CONFIG_ID:               EGLint = 0x3028;
pub const EGL_LEVEL:                   EGLint = 0x3029;
pub const EGL_MAX_PBUFFER_HEIGHT:      EGLint = 0x302A;
pub const EGL_MAX_PBUFFER_PIXELS:      EGLint = 0x302B;
pub const EGL_MAX_PBUFFER_WIDTH:       EGLint = 0x302C;
pub const EGL_NATIVE_RENDERABLE:       EGLint = 0x302D;
pub const EGL_NATIVE_VISUAL_ID:        EGLint = 0x302E;
pub const EGL_NATIVE_VISUAL_TYPE:      EGLint = 0x302F;
pub const EGL_SAMPLES:                 EGLint = 0x3031;
pub const EGL_SAMPLE_BUFFERS:          EGLint = 0x3032;
pub const EGL_SURFACE_TYPE:            EGLint = 0x3033;
pub const EGL_TRANSPARENT_TYPE:        EGLint = 0x3034;
pub const EGL_TRANSPARENT_BLUE_VALUE:  EGLint = 0x3035;
pub const EGL_TRANSPARENT_GREEN_VALUE: EGLint = 0x3036;
pub const EGL_TRANSPARENT_RED_VALUE:   EGLint = 0x3037;
pub const EGL_NONE:                    EGLint = 0x3038; // attrib list terminator
pub const EGL_BIND_TO_TEXTURE_RGB:     EGLint = 0x3039;
pub const EGL_BIND_TO_TEXTURE_RGBA:    EGLint = 0x303A;
pub const EGL_MIN_SWAP_INTERVAL:       EGLint = 0x303B;
pub const EGL_MAX_SWAP_INTERVAL:       EGLint = 0x303C;
pub const EGL_LUMINANCE_SIZE:          EGLint = 0x303D;
pub const EGL_ALPHA_MASK_SIZE:         EGLint = 0x303E;
pub const EGL_COLOR_BUFFER_TYPE:       EGLint = 0x303F;
pub const EGL_RENDERABLE_TYPE:         EGLint = 0x3040;
pub const EGL_MATCH_NATIVE_PIXMAP:     EGLint = 0x3041;  // psseudo-attribute (not queryable)
pub const EGL_CONFORMANT:              EGLint = 0x3042;

// config attribute values
pub const EGL_SLOW_CONFIG:           EGLint = 0x3050;  // CONFIG_CAVEAT value
pub const EGL_NON_CONFORMANT_CONFIG: EGLint = 0x3051;  // CONFIG_CAVEAT value
pub const EGL_TRANSPARENT_RGB:       EGLint = 0x3052;  // TRANSPARENT_TYPE value
pub const EGL_RGB_BUFFER:            EGLint = 0x308E;  // COLOR_BUFFER_TYPE value
pub const EGL_LUMINANCE_BUFFER:      EGLint = 0x308F;  // COLOR_BUFFER_TYPE value

// more config attribute values, for TEXTURE_FORMAT
pub const EGL_NO_TEXTURE:   EGLint = 0x305C;
pub const EGL_TEXTURE_RGB:  EGLint = 0x305D;
pub const EGL_TEXTURE_RGBA: EGLint = 0x305E;
pub const EGL_TEXTURE_2D:   EGLint = 0x305F;

// config attribute mask bits
pub const EGL_PBUFFER_BIT:                 EGLint = 0x0001;  // SURFACE_TYPE mask bits
pub const EGL_PIXMAP_BIT:                  EGLint = 0x0002;  // SURFACE_TYPE mask bits
pub const EGL_WINDOW_BIT:                  EGLint = 0x0004;  // SURFACE_TYPE mask bits
pub const EGL_VG_COLORSPACE_LINEAR_BIT:    EGLint = 0x0020;  // SURFACE_TYPE mask bits
pub const EGL_VG_ALPHA_FORMAT_PRE_BIT:     EGLint = 0x0040;  // SURFACE_TYPE mask bits
pub const EGL_MULTISAMPLE_RESOLVE_BOX_BIT: EGLint = 0x0200;  // SURFACE_TYPE mask bits
pub const EGL_SWAP_BEHAVIOR_PRESERVED_BIT: EGLint = 0x0400;  // SURFACE_TYPE mask bits

pub const EGL_OPENGL_ES_BIT:  EGLint = 0x0001; // RENDERABLE_TYPE mask bits
pub const EGL_OPENVG_BIT:     EGLint = 0x0002; // RENDERABLE_TYPE mask bits
pub const EGL_OPENGL_ES2_BIT: EGLint = 0x0004; // RENDERABLE_TYPE mask bits
pub const EGL_OPENGL_BIT:     EGLint = 0x0008; // RENDERABLE_TYPE mask bits

// QueryString targets
pub const EGL_VENDOR:      EGLint = 0x3053;
pub const EGL_VERSION:     EGLint = 0x3054;
pub const EGL_EXTENSIONS:  EGLint = 0x3055;
pub const EGL_CLIENT_APIS: EGLint = 0x308D;

// QuerySurface / SurfaceAttrib / CreatePbufferSurface targets
pub const EGL_HEIGHT:                EGLint = 0x3056;
pub const EGL_WIDTH:                 EGLint = 0x3057;
pub const EGL_LARGEST_PBUFFER:       EGLint = 0x3058;
pub const EGL_TEXTURE_FORMAT:        EGLint = 0x3080;
pub const EGL_TEXTURE_TARGET:        EGLint = 0x3081;
pub const EGL_MIPMAP_TEXTURE:        EGLint = 0x3082;
pub const EGL_MIPMAP_LEVEL:          EGLint = 0x3083;
pub const EGL_RENDER_BUFFER:         EGLint = 0x3086;
pub const EGL_VG_COLORSPACE:         EGLint = 0x3087;
pub const EGL_VG_ALPHA_FORMAT:       EGLint = 0x3088;
pub const EGL_HORIZONTAL_RESOLUTION: EGLint = 0x3090;
pub const EGL_VERTICAL_RESOLUTION:   EGLint = 0x3091;
pub const EGL_PIXEL_ASPECT_RATIO:    EGLint = 0x3092;
pub const EGL_SWAP_BEHAVIOR:         EGLint = 0x3093;
pub const EGL_MULTISAMPLE_RESOLVE:   EGLint = 0x3099;

// RENDER_BUFFER values / BindTexImage / ReleaseTexImage buffer targets
pub const EGL_BACK_BUFFER:   EGLint = 0x3084;
pub const EGL_SINGLE_BUFFER: EGLint = 0x3085;

// OpenVG color spaces */
pub const EGL_VG_COLORSPACE_sRGB:   EGLint = 0x3089;  // VG_COLORSPACE value
pub const EGL_VG_COLORSPACE_LINEAR: EGLint = 0x308A;  // VG_COLORSPACE value

// OpenVG alpha formats
pub const EGL_VG_ALPHA_FORMAT_NONPRE: EGLint = 0x308B; // ALPHA_FORMAT value
pub const EGL_VG_ALPHA_FORMAT_PRE:    EGLint = 0x308C; // ALPHA_FORMAT value

// constant scale factor by which fractional display resolutions & aspect ratio are scaled when
// queried as integer values
pub const EGL_DISPLAY_SCALING: EGLint = 10000;

// unknown display resolution/aspect ratio
pub const EGL_UNKNOWN: EGLint = -1;

// back buffer swap behaviors
pub const EGL_BUFFER_PRESERVED: EGLint = 0x3094; // SWAP_BEHAVIOR value
pub const EGL_BUFFER_DESTROYED: EGLint = 0x3095; // SWAP_BEHAVIOR value

// CreatePbufferFromClientBuffer buffer types
pub const EGL_OPENVG_IMAGE: EGLint = 0x3096;

// QueryContext targets
pub const EGL_CONTEXT_CLIENT_TYPE: EGLint = 0x3097;

// CreateContext attributes
pub const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;

// multisample resolution behaviors
pub const EGL_MULTISAMPLE_RESOLVE_DEFAULT: EGLint = 0x309A; // MULTISAMPLE_RESOLVE value
pub const EGL_MULTISAMPLE_RESOLVE_BOX:     EGLint = 0x309B; // MULTISAMPLE_RESOLVE value

// BindAPI/QueryAPI targets
pub const EGL_OPENGL_ES_API: EGLenum = 0x30A0;
pub const EGL_OPENVG_API:    EGLenum = 0x30A1;
pub const EGL_OPENGL_API:    EGLenum = 0x30A2;

// GetCurrentSurface targets
pub const EGL_DRAW: EGLint = 0x3059;
pub const EGL_READ: EGLint = 0x305A;

// WaitNative engines
pub const EGL_CORE_NATIVE_ENGINE: EGLint = 0x305B;

// EGL 1.2 tokens renamed for consistency in EGL 1.3
pub const EGL_COLORSPACE:          EGLint = EGL_VG_COLORSPACE;
pub const EGL_ALPHA_FORMAT:        EGLint = EGL_VG_ALPHA_FORMAT;
pub const EGL_COLORSPACE_sRGB:     EGLint = EGL_VG_COLORSPACE_sRGB;
pub const EGL_COLORSPACE_LINEAR:   EGLint = EGL_VG_COLORSPACE_LINEAR;
pub const EGL_ALPHA_FORMAT_NONPRE: EGLint = EGL_VG_ALPHA_FORMAT_NONPRE;
pub const EGL_ALPHA_FORMAT_PRE:    EGLint = EGL_VG_ALPHA_FORMAT_PRE;

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------


// ----------
// Configuration Management
// ----------
#[no_mangle]
pub extern "C" fn eglGetConfigs(dpy: EGLDisplay, configs: EGLConfig,
                     config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig,
                          attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint,
                       configs: *mut EGLConfig, config_size: EGLint,
                       num_config: *mut EGLint) -> EGLBoolean;


// ----------
// Rendering Surfaces
// ----------
#[no_mangle]
pub extern "C" fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig,
                              win: EGLNativeWindowType,
                              attrib_list: *const EGLint) -> EGLSurface;

#[no_mangle]
pub extern "C" fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig,
                               attrib_list: *const EGLint) -> EGLSurface;

#[no_mangle]
pub extern "C" fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum,
                                        buffer: EGLClientBuffer, config: EGLConfig,
                                        attrib_list: *const EGLint) -> EGLSurface;

#[no_mangle]
pub extern "C" fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig,
                              pixmap: EGLNativePixmapType,
                              attrib_list: *const EGLint) -> EGLSurface;

#[no_mangle]
pub extern "C" fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface,
                        attribute: EGLint, value: EGLint) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface,
                       attribute: EGLint, value: *mut EGLint) -> EGLBoolean;


// ----------
// Rendering Contexts
// ----------
#[no_mangle]
pub extern "C" fn eglBindAPI(api: EGLenum) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglQueryAPI() -> EGLenum;

#[no_mangle]
pub extern "C" fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig,
                        share_context: EGLContext,
                        attrib_list: *const EGLint) -> EGLContext;

#[no_mangle]
pub extern "C" fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface,
                      read: EGLSurface, ctx: EGLContext) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglGetCurrentContext() -> EGLContext;

#[no_mangle]
pub extern "C" fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;

#[no_mangle]
pub extern "C" fn eglGetCurrentDisplay() -> EGLDisplay;

#[no_mangle]
pub extern "C" fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext,
                       attribute: EGLint, value: *mut EGLint) -> EGLBoolean;


// ----------
// Initialization and Terminating
// ----------
#[no_mangle]
pub extern "C" fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;

#[no_mangle]
pub extern "C" fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char;

#[no_mangle]
pub extern "C" fn eglReleaseThread() -> EGLBoolean;


// ----------
// Synchronization Primitives
// ----------
#[no_mangle]
pub extern "C" fn eglWaitClient() -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglWaitGL() -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglWaitNative(engine: EGLint) -> EGLBoolean;


// ----------
// Posting the Color Buffer
// ----------
#[no_mangle]
pub extern "C" fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface,
                      target: EGLNativePixmapType) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;


// ----------
// Render to Textures
// ----------
#[no_mangle]
pub extern "C" fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;

#[no_mangle]
pub extern "C" fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface,
                          buffer: EGLint) -> EGLBoolean;


// ----------
// Obtain Extension Function Pointers
// ----------
#[no_mangle]
pub extern "C" fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn();


// ----------
// Errors
// ----------
#[no_mangle]
pub extern "C" fn eglGetError() -> EGLint;
