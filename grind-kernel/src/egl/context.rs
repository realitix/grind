use opengl::gles2::ContextGlES2;

use egl::surface::Surface;

pub struct Context {
    // if read surface is none, it's the same as draw surface
    read_surface: Option<Surface>,
    draw_surface: Option<Surface>,
    gl_context: ContextGlES2,
}

impl PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        self as *const Context == other as *const Context
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            read_surface: None,
            draw_surface: None,
            gl_context: ContextGlES2::new(),
        }
    }

    pub fn get_gl_context(&mut self) -> &mut ContextGlES2 {
        &mut self.gl_context
    }

    pub fn set_surfaces(&mut self, draw: Option<Surface>, read: Option<Surface>) {
        self.read_surface = read;
        self.draw_surface = draw;
    }

    pub fn swap_buffers(&self) {
        self.draw_surface.as_ref().unwrap().swap_buffers();
    }
}
