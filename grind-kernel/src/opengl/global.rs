use opengl::types::*;
use std::cell::RefCell;

thread_local! {
    pub static GL_ERROR: RefCell<GLenum> = RefCell::new(NO_ERROR);
}

pub fn GL_RESULT(code: GLenum) {
    GL_ERROR.with(|c| {
        *c.borrow_mut() = code;
    });
}
