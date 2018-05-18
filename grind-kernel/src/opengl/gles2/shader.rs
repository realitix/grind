use opengl::types::*;


pub struct Shader {
    pub id: GLuint,
    shader_type: GLenum
}

impl Shader {
    pub fn new(id: GLuint, shader_type: GLenum) -> Shader {
        Shader {
            id,
            shader_type
        }
    }

    pub fn set_source(&mut self, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
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
