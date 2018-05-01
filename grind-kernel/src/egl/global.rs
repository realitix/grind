use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

use egl::context::LocalContext;
use egl::display::Display;
use egl::types::*;

lazy_static! {
    pub static ref DISPLAYS: RwLock<Vec<Display>> = RwLock::new(Vec::new());
}

thread_local! {
    pub static CONTEXT: RefCell<Option<LocalContext>> = RefCell::new(None);
    pub static LAST_EGL_CALL: RefCell<EGLint> = RefCell::new(EGL_SUCCESS);
}

pub fn EGL_RESULT(code: EGLint) -> EGLBoolean {
    LAST_EGL_CALL.with(|c| {
        *c.borrow_mut() = code;
    });

    match code {
        EGL_SUCCESS => EGL_TRUE,
        _ => EGL_FALSE,
    }
}
