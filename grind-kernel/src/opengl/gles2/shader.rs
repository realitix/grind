use opengl::types::{GLuint, GLenum};


pub struct Shader {
    id: GLuint,
    shader_type: GLenum
}

impl Shader {
    pub fn new(id: GLuint, shader_type: GLenum) -> Shader {
        Shader {
            id,
            shader_type
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
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
