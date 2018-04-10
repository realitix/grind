use egl::surface::Surface;

pub struct Context {
    surface: Option<Surface>,
}

impl Context {
    pub fn new(surface: Option<Surface>) -> Context {
        Context { surface }
    }
}
