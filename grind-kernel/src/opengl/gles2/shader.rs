use std::ffi::CStr;
use std::mem;

use glsltranspiler::{transpile, ShaderType};

use opengl::types::*;

pub struct Shader {
    pub id: GLuint,
    shader_type: GLenum,
    source: String,
    source_transpiled: Option<String>
}

impl Shader {
    pub fn new(id: GLuint, shader_type: GLenum) -> Shader {
        Shader {
            id,
            shader_type,
            source: String::new(),
            source_transpiled: None
        }
    }

    pub fn set_source(
        &mut self,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        let mut result = String::new();
        for i in 0..count {
            let slice = unsafe { CStr::from_ptr(*string.offset(i as isize)) };
            let clean_slice = slice.to_str().unwrap();
            result.push_str(clean_slice);
            result.push('\n');
        }

        mem::replace(&mut self.source, result);
    }

    pub fn compile(&mut self) {
        let shader_type = match self.shader_type {
            VERTEX_SHADER => ShaderType::Vertex,
            FRAGMENT_SHADER => ShaderType::Fragment,
            _ => panic!("Unknow shader type")
        };

        // TODO: 1. check shader validity
        // 2. transpile shader to version 450
        let transpilation = transpile(&self.source, shader_type);
        self.source_transpiled = Some(transpilation.text);
        // 3. compile to spirv
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
    vertex: Option<Shader>,
    fragment: Option<Shader>,
}

impl ShaderProgram {
    pub fn new(id: GLuint) -> ShaderProgram {
        ShaderProgram {
            id: id,
            vertex: None,
            fragment: None,
        }
    }
}
