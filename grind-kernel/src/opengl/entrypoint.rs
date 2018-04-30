use egl::global::CONTEXT;

use opengl::types::*;
use opengl::gles2::ContextGlES2;

fn with_gl<F>(f: F)
where
    F: FnOnce(&mut ContextGlES2),
{
    CONTEXT.with(|c| {
        c.borrow_mut().as_mut().unwrap().with_gl_context(|gl| {
            f(gl);
        });
    });
}

pub fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    with_gl(|gl| {
        gl.clear_color(red, green, blue, alpha);
    });
}

pub fn clear(mask: GLbitfield) {
    with_gl(|gl| {
        gl.clear(mask);
    });
}