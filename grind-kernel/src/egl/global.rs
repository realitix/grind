use egl::types::*;
use egl::display::Display;
use egl::surface::Surface;
use egl::context::Context;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

lazy_static! {
    pub static ref DISPLAYS: RwLock<Vec<Display>> = RwLock::new(Vec::new());
    pub static ref SURFACES: Mutex<Vec<Surface>> = Mutex::new(Vec::new());
    pub static ref CONTEXTS: Mutex<Vec<Context>> = Mutex::new(Vec::new());
}

thread_local! {
    pub static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
    pub static LAST_EGL_CALL: RefCell<EGLint> = RefCell::new(EGL_SUCCESS);
}

pub fn EGL_RESULT(code: EGLint) -> EGLBoolean {
    LAST_EGL_CALL.with(|r| {
        *r.borrow_mut() = code;
    });

    match code {
        EGL_SUCCESS => EGL_TRUE,
        _ => EGL_FALSE,
    }
}
