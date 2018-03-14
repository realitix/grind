use std::collections::HashMap;
use std::thread;
use std::sync::Mutex;

use egl::types::*;
use egl::display::Display;
use egl::display::is_available;

use kernel::vulkan::VulkanDriver;


// TODO: Multithreading access
lazy_static! {
    //static ref contexts: HashMap<thread::ThreadId, Context> = HashMap::new();
    static ref CONTEXTS: Mutex<HashMap<thread::ThreadId, Context>> = Mutex::new(HashMap::new());
}


fn get_context() -> &'static Context {
    let thread_id = thread::current().id();
    match CONTEXTS.lock().unwrap().get(&thread_id) {
        Some(context) => &context,
        None => {
            let context = Context::new();
            CONTEXTS.lock().unwrap().insert(thread_id, context);
            CONTEXTS.lock().unwrap().get(&thread_id).unwrap()
        }
    }
}

struct Context {
    display: Display
}


impl Context {
    pub fn new() -> Context {
        Context {
            display: Display::new()
        }
    }
}


pub struct EGL {
}


impl EGL {
    pub fn get_display(display_id: EGLNativeDisplayType) -> EGLDisplay {
        match is_available() {
            false => EGL_NO_DISPLAY,
            true => {
                &(get_context().display) as *const Display as EGLDisplay
            }
        }
    }
}
