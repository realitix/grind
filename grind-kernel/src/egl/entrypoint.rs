use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

use kernel::vulkan::VulkanDriver;

use egl::types::*;
use egl::display::{is_available, Display};
use egl::context::Context;
use egl::wayland::WaylandDisplay;

static EGL_VERSION_MAJOR: EGLint = 1;
static EGL_VERSION_MINOR: EGLint = 4;

lazy_static! {
    static ref DISPLAYS: RwLock<Vec<Display>> = RwLock::new(Vec::new());
    static ref CONTEXTS: Mutex<Vec<Context>> = Mutex::new(Vec::new());
}

thread_local! {
    static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
    static LAST_EGL_CALL: RefCell<EGLint> = RefCell::new(EGL_SUCCESS);
}

fn with_display<F>(egl_display: EGLDisplay, f: F) -> bool
where
    F: FnOnce(&Display) -> bool,
{
    let lock = DISPLAYS.read().unwrap();
    let mut current_display: Option<&Display> = None;

    for display in lock.iter() {
        if display as *const Display as EGLDisplay == egl_display {
            current_display = Some(display);
        }
    }

    match current_display {
        Some(d) => f(d),
        None => false,
    }
}

pub fn get_display(display_id: EGLNativeDisplayType) -> EGLDisplay {
    match is_available() {
        false => EGL_NO_DISPLAY,
        true => {
            let d = Display::new(WaylandDisplay::new(display_id), VulkanDriver::new());
            {
                DISPLAYS.write().unwrap().push(d);
            }
            {
                DISPLAYS.read().unwrap().last().unwrap() as *const Display as EGLDisplay
            }
        }
    }
}

pub fn initialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean {
    match with_display(dpy, |d| {
        unsafe {
            if !major.is_null() {
                *major = EGL_VERSION_MAJOR;
            }
            if !minor.is_null() {
                *minor = EGL_VERSION_MINOR;
            }
        }
        true
    }) {
        false => EGL_FALSE,
        true => EGL_TRUE,
    }
}

pub fn test_current(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) {
    CONTEXT.with(|c| {
        let mut lock = CONTEXTS.lock().unwrap();
        let mut target: Option<usize> = None;
        for (i, elem) in lock.iter().enumerate() {
            if elem as *const Context as EGLContext == ctx {
                target = Some(i);
            }
        }

        if target.is_some() {
            *c.borrow_mut() = Some(lock.remove(target.unwrap()));
        }
    });
}
