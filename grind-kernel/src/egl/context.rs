use egl::surface::Surface;

pub struct Context {
    read_surface: Option<Surface>,
    draw_surface: Option<Surface>,
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
        }
    }

    pub fn set_surfaces(&mut self, draw: Option<Surface>, read: Option<Surface>) {
        self.read_surface = read;
        self.draw_surface = draw;
    }
}
