use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

use egl::types::*;
use egl::display::Display;
use egl::surface::Surface;
use egl::context::Context;

lazy_static! {
    pub static ref DISPLAYS: RwLock<Vec<Display>> = RwLock::new(Vec::new());
}

#[thread_local]
pub static mut CONTEXT: Option<Context> = None;

#[thread_local]
pub static LAST_EGL_CALL: EGLint = EGL_SUCCESS;

pub fn EGL_RESULT(code: EGLint) -> EGLBoolean {
    unsafe { LAST_EGL_CALL = code };

    match code {
        EGL_SUCCESS => EGL_TRUE,
        _ => EGL_FALSE,
    }
}
