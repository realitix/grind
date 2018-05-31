use egl::global::CONTEXT;

use opengl::gles2::ContextGlES2;
use opengl::global::GL_ERROR;
use opengl::types::*;

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

pub fn get_error() -> GLenum {
    GL_ERROR.with(|c| *c.borrow())
}

pub fn create_program() -> GLuint {
    let mut result = None;
    with_gl(|gl| {
        result = Some(gl.create_program());
    });

    match result {
        Some(r) => r,
        None => 0,
    }
}

pub fn create_shader(_type: GLenum) -> GLuint {
    let mut result = None;
    with_gl(|gl| {
        result = Some(gl.create_shader(_type));
    });

    match result {
        Some(r) => r,
        None => 0,
    }
}

pub fn shader_source(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    with_gl(|gl| {
        gl.shader_source(shader, count, string, length);
    });
}

pub fn compile_shader(shader: GLuint) {
    with_gl(|gl| {
        gl.compile_shader(shader);
    });
}

pub fn get_shaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    with_gl(|gl| {
        gl.get_shaderiv(shader, pname, params);
    });
}

pub fn attach_shader(program: GLuint, shader: GLuint) {
    with_gl(|gl| {
        gl.attach_shader(program, shader);
    });
}

pub fn link_program(program: GLuint) {
    with_gl(|gl| {
        gl.link_program(program);
    });
}
