use std::ffi::CStr;
use std::mem;

use opengl::types::*;


pub struct Shader {
    pub id: GLuint,
    shader_type: GLenum,
    source: String
}

impl Shader {
    pub fn new(id: GLuint, shader_type: GLenum) -> Shader {
        Shader {
            id,
            shader_type,
            source: String::new()
        }
    }

    pub fn set_source(&mut self, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
        let mut result = String::new();
        for i in 0..count {
            let slice = unsafe { CStr::from_ptr(*string.offset(i as isize)) };
            let clean_slice = slice.to_str().unwrap();
            result.push_str(clean_slice);
            result.push('\n');
        }

        mem::replace(&mut self.source, result);
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
    vertex: Option<Shader>,
    fragment: Option<Shader>
}

impl ShaderProgram {
    pub fn new(id: GLuint) -> ShaderProgram {
        ShaderProgram {
            id: id,
            vertex: None,
            fragment: None
        }
    }
}
