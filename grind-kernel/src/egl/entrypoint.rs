use std::sync::Arc;
use std::ptr::Unique;

use kernel::vulkan::VulkanDriver;

use egl::types::*;
use egl::global::*;
use egl::display::{is_available, Display};
use egl::config::Config;
use egl::context::{GlobalContext, LocalContext};
use egl::surface::{GlobalSurface, LocalSurface};
use egl::wayland::WaylandDisplay;

static EGL_VERSION_MAJOR: EGLint = 1;
static EGL_VERSION_MINOR: EGLint = 4;

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
        Some(d) => {
            if d.configs.len() == 0 {
                return EGL_RESULT(EGL_NOT_INITIALIZED);
            }

            f(d)
        }
        None => EGL_RESULT(EGL_BAD_DISPLAY),
    }
}

fn with_mut_display<F>(egl_display: EGLDisplay, f: F) -> EGLBoolean
where
    F: FnOnce(&mut Display) -> EGLBoolean,
{
    let mut lock = DISPLAYS.write().unwrap();
    let mut current_display: Option<&mut Display> = None;

    for display in lock.iter_mut() {
        if display as *mut Display as EGLDisplay == egl_display {
            current_display = Some(display);
        }
    }

    match current_display {
        Some(d) => {
            if d.configs.len() == 0 {
                return EGL_RESULT(EGL_NOT_INITIALIZED);
            }

            f(d)
        }
        None => EGL_RESULT(EGL_BAD_DISPLAY),
    }
}

pub fn get_display(display_id: EGLNativeDisplayType) -> EGLDisplay {
    match is_available() {
        false => EGL_NO_DISPLAY,
        true => {
            let d = Display::new(WaylandDisplay::new(display_id));
            let mut lock = DISPLAYS.write().unwrap();
            lock.push(d);
            lock.last().unwrap() as *const Display as EGLDisplay
        }
    }
}

pub fn initialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean {
    let mut lock = DISPLAYS.write().unwrap();
    let mut current_display: Option<&mut Display> = None;

    for display in lock.iter_mut() {
        if display as *const Display as EGLDisplay == dpy {
            current_display = Some(display);
        }
    }

    match current_display {
        None => EGL_RESULT(EGL_BAD_DISPLAY),
        Some(d) => {
            d.initialize();
            unsafe {
                if !major.is_null() {
                    *major = EGL_VERSION_MAJOR;
                }
                if !minor.is_null() {
                    *minor = EGL_VERSION_MINOR;
                }
            }
            EGL_RESULT(EGL_SUCCESS)
        }
    }
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
        // Check num_config not NULL
        let ptr = unsafe { num_config.as_ref() };
        if ptr.is_none() {
            return EGL_RESULT(EGL_BAD_PARAMETER);
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

        EGL_RESULT(EGL_SUCCESS)
    })
}

pub fn get_config_attrib(
    dpy: EGLDisplay,
    egl_config: EGLConfig,
    attribute: EGLint,
    value: *mut EGLint,
) -> EGLBoolean {
    with_display(dpy, |d| {
        d.with_config(egl_config, |c| {
            let res = c.get_attrib(attribute);
            match res {
                None => EGL_RESULT(EGL_BAD_CONFIG),
                Some(a) => {
                    unsafe {
                        *value = a as EGLint;
                    }
                    EGL_RESULT(EGL_SUCCESS)
                }
            }
        })
    })
}

pub fn choose_config(
    dpy: EGLDisplay,
    attrib_list: *const EGLint,
    configs: *mut EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> EGLBoolean {
    with_display(dpy, |d| {
        // TODO: Currently just a copy of get_configs
        // because it's just for the POC
        //
        // Check num_config not NULL
        let ptr = unsafe { num_config.as_ref() };
        if ptr.is_none() {
            return EGL_RESULT(EGL_BAD_PARAMETER);
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

        EGL_RESULT(EGL_SUCCESS)
    })
}

pub fn create_window_surface(
    dpy: EGLDisplay,
    config: EGLConfig,
    win: EGLNativeWindowType,
    attrib_list: *const EGLint,
) -> EGLSurface {
    let mut surface_pointer: Option<EGLSurface> = None;
    with_mut_display(dpy, |d| {
        let rwin = Arc::new(Unique::new(win).expect("Win unavailable"));
        let surface = GlobalSurface::new(Arc::clone(&d.native_display.display_id), rwin);
        surface_pointer = Some(d.add_surface(surface));
        EGL_TRUE
        // TODO: Error management
    });

    match surface_pointer {
        Some(p) => p,
        None => EGL_NO_SURFACE,
    }
}

pub fn create_context(
    dpy: EGLDisplay,
    egl_config: EGLConfig,
    share_context: EGLContext,
    attrib_list: *const EGLint,
) -> EGLContext {
    let mut context_pointer: Option<EGLContext> = None;
    with_mut_display(dpy, |d| {
        let context = GlobalContext::new();
        context_pointer = Some(d.add_context(context));
        EGL_TRUE
    });

    match context_pointer {
        Some(p) => p,
        None => EGL_NO_CONTEXT,
    }
}

pub fn make_current(
    dpy: EGLDisplay,
    draw: EGLSurface,
    read: EGLSurface,
    ctx: EGLContext,
) -> EGLBoolean {
    with_mut_display(dpy, |d| {
        // Check surface
        if !d.is_surface(draw) || !d.is_surface(read) {
            // TODO EGL_RESULT
            return EGL_FALSE;
        }

        // Get surfaces and context
        let draw_surface = Some(d.drain_surface(draw));
        let read_surface = {
            match draw == read {
                true => None,
                false => Some(d.drain_surface(read)),
            }
        };
        // TODO: GlobalContext became Useless
        let mut context = d.drain_context(ctx);

        // Create LocalContext
        let local_context = LocalContext::new(LocalSurface::new(&draw_surface.unwrap()));

        // Put context in local thread
        CONTEXT.with(|c| {
            *c.borrow_mut() = Some(local_context);
        });

        EGL_TRUE
    })
}

pub fn swap_buffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    // Currently, I don't use the provided surface, I take the one from the current context
    CONTEXT.with(|c| {
        c.borrow().as_ref().unwrap().swap_buffers();
    });

    EGL_TRUE
}
