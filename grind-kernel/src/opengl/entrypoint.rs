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

pub fn get_programiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    with_gl(|gl| {
        gl.get_programiv(program, pname, params);
    });
}

pub fn delete_shader(shader: GLuint) {
    with_gl(|gl| {
        gl.delete_shader(shader);
    });
}

pub fn gen_buffers(n: GLsizei, buffers: *mut GLuint) {
    with_gl(|gl| {
        gl.gen_buffers(n, buffers);
    });
}

pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    with_gl(|gl| {
        gl.bind_buffer(target, buffer);
    });
}

pub fn buffer_data(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum) {
    with_gl(|gl| {
        gl.buffer_data(target, size, data, usage);
    });
}

pub fn enable_vertex_attrib_array(index: GLuint) {
    with_gl(|gl| {
        gl.enable_vertex_attrib_array(index);
    });
}

pub fn get_attrib_location(program: GLuint, name: *const GLchar) -> GLint {
    let mut res = None;
    with_gl(|gl| {
        res = Some(gl.get_attrib_location(program, name));
    });
    res.unwrap()
}

pub fn vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    _type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    ptr: *const GLvoid,
) {
    with_gl(|gl| {
        gl.vertex_attrib_pointer(index, size, _type, normalized, stride, ptr);
    });
}

pub fn use_program(program: GLuint) {
    with_gl(|gl| {
        gl.use_program(program);
    });
}

pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    with_gl(|gl| {
        gl.draw_arrays(mode, first, count);
    });
}
