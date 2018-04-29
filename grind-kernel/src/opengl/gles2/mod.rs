use opengl::types::*;

pub struct ContextGlES2 {
    clear_color: [GLclampf; 4],
}

impl ContextGlES2 {
    pub fn new() -> ContextGlES2 {
        ContextGlES2 {
            clear_color: [0.; 4],
        }
    }

    pub fn clear_color(&mut self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        self.clear_color = [red, green, blue, alpha]
    }
}
