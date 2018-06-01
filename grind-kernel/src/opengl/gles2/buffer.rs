use opengl::types::*;

pub struct Buffer {
    id: GLuint,
}

impl Buffer {
    pub fn new(id: GLuint) -> Buffer {
        Buffer { id }
    }
}
