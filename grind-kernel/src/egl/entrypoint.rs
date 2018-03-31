use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

use kernel::vulkan::VulkanDriver;

use egl::types::*;
use egl::display::{is_available, Display};
use egl::config::Config;
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

fn set_result(code: EGLint) -> EGLBoolean {
    LAST_EGL_CALL.with(|r| {
        *r.borrow_mut() = code;
    });

    match code {
        EGL_SUCCESS => EGL_TRUE,
        _ => EGL_FALSE,
    }
}

fn with_display<F>(egl_display: EGLDisplay, f: F) -> EGLBoolean
where
    F: FnOnce(&Display) -> EGLBoolean,
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
        None => set_result(EGL_BAD_DISPLAY),
    }
}

fn with_mutable_display<F>(egl_display: EGLDisplay, f: F) -> EGLBoolean
where
    F: FnOnce(&mut Display) -> EGLBoolean,
{
    let mut lock = DISPLAYS.write().unwrap();
    let mut current_display: Option<&mut Display> = None;

    for display in lock.iter_mut() {
        if display as *const Display as EGLDisplay == egl_display {
            current_display = Some(display);
        }
    }

    match current_display {
        Some(d) => f(d),
        None => set_result(EGL_BAD_DISPLAY),
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
    with_mutable_display(dpy, |d| {
        d.initialize();
        EGL_TRUE
    });

    unsafe {
        if !major.is_null() {
            *major = EGL_VERSION_MAJOR;
        }
        if !minor.is_null() {
            *minor = EGL_VERSION_MINOR;
        }
    }

    set_result(EGL_SUCCESS)
}

pub fn get_error() -> EGLint {
    LAST_EGL_CALL.with(|c| *c.borrow())
}

pub fn get_configs(
    dpy: EGLDisplay,
    configs: *mut EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> EGLBoolean {
    with_display(dpy, |d| {
        // Check display initialized
        if d.configs.len() == 0 {
            return set_result(EGL_NOT_INITIALIZED);
        }

        // Check num_config not NULL
        let ptr = unsafe { num_config.as_ref() };
        if ptr.is_none() {
            return set_result(EGL_BAD_PARAMETER);
        }

        // Fill config if config is not NULL
        let ptr = unsafe { configs.as_ref() };
        if !ptr.is_none() {
            let max_config_size = {
                match config_size as usize > d.configs.len() {
                    true => d.configs.len() as isize,
                    false => config_size as isize,
                }
            };

            for i in 0..max_config_size {
                unsafe {
                    *configs.offset(i) = &d.configs[i as usize] as *const Config as EGLConfig;
                }
            }
        }

        unsafe {
            *num_config = d.configs.len() as EGLint;
        }

        set_result(EGL_SUCCESS)
    })
}

pub fn get_config_attrib(
    dpy: EGLDisplay,
    egl_config: EGLConfig,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    with_display(dpy, |d| {
        // Check display initialized
        if d.configs.len() == 0 {
            return set_result(EGL_NOT_INITIALIZED);
        }

        let attr = d.get_config_attrib(egl_config, attribute);

        match attr {
            None => set_result(EGL_BAD_CONFIG),
            Some(a) => {
                unsafe {
                    *value = a as EGLint;
                }
                set_result(EGL_SUCCESS)
            }
        }
    })
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
