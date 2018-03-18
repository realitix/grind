use std::cell::RefCell;
use std::sync::Mutex;

use egl::types::*;
use egl::display::Display;
use egl::display::is_available;

use kernel::vulkan::VulkanDriver;

lazy_static! {
    static ref DISPLAYS: Mutex<Vec<Display>> = Mutex::new(Vec::new());
}

thread_local! {
    static DISPLAY: RefCell<Option<Display>> = RefCell::new(None);
}

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
                DISPLAYS.lock().unwrap().push(Display::new());
                0 as EGLDisplay
            }
        }
    }
}
