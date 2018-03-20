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

/* TODELETE
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
*/

pub struct EGL {}

impl EGL {
    pub fn get_display(display_id: EGLNativeDisplayType) -> EGLDisplay {
        match is_available() {
            false => EGL_NO_DISPLAY,
            true => {
                let d = Display::new();
                { DISPLAYS.lock().unwrap().push(d); }
                { DISPLAYS.lock().unwrap().last().unwrap() as *const Display as EGLDisplay }
            }
        }
    }

    pub fn test_current(display: EGLDisplay) {
        DISPLAY.with(|d| {
            let mut lock = DISPLAYS.lock().unwrap();
            let mut target: i32 = -1;
            for (i, elem) in lock.iter().enumerate() {
                if elem as *const Display as EGLDisplay == display {
                    target = i as i32;
                }
            }
            if target >= 0 {
                *d.borrow_mut() = Some(lock.remove(target as usize));
            }
        });
    }
}
