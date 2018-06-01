use opengl::types::*;

pub struct Buffer {
    pub id: GLuint,
    pub target: GLenum,
}

impl Buffer {
    pub fn new(id: GLuint) -> Buffer {
        Buffer { id, target: 0 }
    }
}
