use std::cell::RefCell;
use std::sync::Mutex;

use kernel::vulkan::VulkanDriver;

use egl::types::*;
use egl::display::{is_available, Display};
use egl::context::Context;
use egl::wayland::WaylandDisplay;

lazy_static! {
    static ref DISPLAYS: Mutex<Vec<Display>> = Mutex::new(Vec::new());
}

lazy_static! {
    static ref CONTEXTS: Mutex<Vec<Context>> = Mutex::new(Vec::new());
}

thread_local! {
    static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
}

pub struct EGL {}

impl EGL {
    pub fn get_display(display_id: EGLNativeDisplayType) -> EGLDisplay {
        match is_available() {
            false => EGL_NO_DISPLAY,
            true => {
                let d = Display::new(WaylandDisplay::new(display_id), VulkanDriver::new());
                {
                    DISPLAYS.lock().unwrap().push(d);
                }
                {
                    DISPLAYS.lock().unwrap().last().unwrap() as *const Display as EGLDisplay
                }
            }
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
}
