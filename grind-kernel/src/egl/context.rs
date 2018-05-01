use egl::surface::{GlobalSurface, LocalSurface};
use kernel::vulkan::VulkanDriver;
use opengl::gles2::ContextGlES2;
use std::sync::Arc;

// GlobalContext to be stored in a Display
pub struct GlobalContext {
    // if read surface is none, it's the same as draw surface
    read_surface: Option<GlobalSurface>,
    draw_surface: Option<GlobalSurface>,
}

impl PartialEq for GlobalContext {
    fn eq(&self, other: &GlobalContext) -> bool {
        self as *const GlobalContext == other as *const GlobalContext
    }
}

impl GlobalContext {
    pub fn new() -> GlobalContext {
        GlobalContext {
            read_surface: None,
            draw_surface: None,
        }
    }

    pub fn set_surfaces(&mut self, draw: Option<GlobalSurface>, read: Option<GlobalSurface>) {
        self.read_surface = read;
        self.draw_surface = draw;
    }
}

// LocalContext to be strored in the local thread
pub struct LocalContext {
    //draw_surface: LocalSurface,
    gl_context: ContextGlES2,
}

impl LocalContext {
    pub fn new(kernel: VulkanDriver) -> LocalContext {
        let gl_context = ContextGlES2::new(kernel);

        LocalContext {
            //      draw_surface,
            gl_context,
        }
    }

    pub fn with_gl_context<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ContextGlES2),
    {
        f(&mut self.gl_context);
    }

    pub fn swap_buffers(&mut self) {
        self.gl_context.swap_buffers();
    }
}
