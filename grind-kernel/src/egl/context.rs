use std::cell::RefCell;

use egl::types::*;
use egl::display::Display;
use egl::display::is_available;

use kernel::vulkan::VulkanDriver;

thread_local!(static CONTEXT: RefCell<Context> = RefCell::new(Context::new()));

struct Context {
    display: Option<Display>,
}

impl Context {
    pub fn new() -> Context {
        Context { display: None }
    }

    pub fn init(&mut self) {
        self.display = Some(Display::new());
    }
}

pub struct EGL {}

impl EGL {
    pub fn get_display(display_id: EGLNativeDisplayType) -> EGLDisplay {
        match is_available() {
            false => EGL_NO_DISPLAY,
            true => {
                CONTEXT.with(|c| {
                    c.borrow_mut().init();
                    // Note: I don't manage to send the Display pointer so I send the Context
                    // pointer. It should be the same.
                    &(*c.borrow()) as *const Context as EGLDisplay
                })
            }
        }
    }
}
