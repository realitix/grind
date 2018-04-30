use opengl::gles2::ContextGlES2;

use egl::surface::Surface;

pub struct Context {
    // if read surface is none, it's the same as draw surface
    read_surface: Option<Surface>,
    draw_surface: Option<Surface>,
    gl_context: Option<ContextGlES2>,
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
            gl_context: None,
        }
    }

    pub fn with_gl_context<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ContextGlES2),
    {
        f(self.gl_context.as_mut().unwrap());
    }

    pub fn set_surfaces(&mut self, draw: Option<Surface>, read: Option<Surface>) {
        self.read_surface = read;
        self.draw_surface = draw;

        // we can create context with driver
        let gl_context = ContextGlES2::new(self.draw_surface.as_ref().unwrap().clone_kernel());
        self.gl_context = Some(gl_context);
    }

    pub fn swap_buffers(&self) {
        self.draw_surface.as_ref().unwrap().swap_buffers();
    }
}
