use egl::global::CONTEXT;

use opengl::types::*;

pub fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    CONTEXT.with(|c| {
        c.borrow_mut()
            .as_mut()
            .unwrap()
            .get_gl_context()
            .clear_color(red, green, blue, alpha);
    });
}
