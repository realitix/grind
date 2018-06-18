use std::sync::Arc;
use std::ptr::Unique;
use libc::c_void;

use kernel::vulkan::is_available as vulkan_is_available;
use kernel::Kernel;

use egl::config::Config;
use egl::context::GlobalContext;
use egl::global::EGL_RESULT;
use egl::platform::wayland::WaylandDisplay;
use egl::surface::GlobalSurface;
use egl::types::*;

pub fn is_available() -> bool {
    vulkan_is_available()
}

pub struct Display {
    pub native_display: WaylandDisplay,
    pub configs: Vec<Config>,
    surfaces: Vec<GlobalSurface>,
    contexts: Vec<GlobalContext>,
}

impl Display {
    pub fn new(native_display: WaylandDisplay) -> Display {
        Display {
            native_display,
            configs: Vec::new(),
            surfaces: Vec::new(),
            contexts: Vec::new(),
        }
    }

    pub fn initialize(&mut self) -> bool {
        self.configs.push(Config {
            red_size: 8,
            green_size: 8,
            blue_size: 8,
            alpha_size: 8,
            buffer_size: 32,
            depth_size: 24,
            stencil_size: 8,
            alpha_mask_size: 0,
            bind_to_texture_rgb: 0,
            bind_to_texture_rgba: 0,
            color_buffer_type: EGL_RGB_BUFFER,
            config_caveat: EGL_NONE,
            config_id: 1,
            conformant: EGL_OPENGL_ES2_BIT,
            level: 0,
            luminance_size: 0,
            max_pbuffer_width: 4096,
            max_pbuffer_height: 4096,
            max_pbuffer_pixels: 0,
            max_swap_interval: 1,
            min_swap_interval: 0,
            native_renderable: 1,
            native_visual_id: EGL_NONE,
            native_visual_type: EGL_NONE,
            renderable_type: EGL_OPENGL_ES_BIT,
            sample_buffers: 0,
            samples: 0,
            surface_type: EGL_WINDOW_BIT,
            transparent_type: EGL_NONE,
            transparent_red_value: 0,
            transparent_green_value: 0,
            transparent_blue_value: 0,
        });

        return true;
    }

    pub fn get_config(&self, egl_config: EGLConfig) -> &Config {
        for config in self.configs.iter() {
            if config as *const Config as EGLConfig == egl_config {
                return config;
            }
        }
        panic!("Can't get config");
    }

    pub fn with_config<F>(&self, egl_config: EGLConfig, f: F) -> EGLBoolean
    where
        F: FnOnce(&Config) -> EGLBoolean,
    {
        let mut current_config: Option<&Config> = None;

        for config in self.configs.iter() {
            if config as *const Config as EGLConfig == egl_config {
                current_config = Some(config);
            }
        }

        match current_config {
            None => EGL_RESULT(EGL_BAD_CONFIG),
            Some(c) => f(c),
        }
    }

    pub fn add_surface(&mut self, surface: GlobalSurface) -> EGLSurface {
        self.surfaces.push(surface);
        self.surfaces.last().unwrap() as *const GlobalSurface as EGLSurface
    }

    pub fn add_context(&mut self, context: GlobalContext) -> EGLContext {
        self.contexts.push(context);
        self.contexts.last().unwrap() as *const GlobalContext as EGLContext
    }

    pub fn drain_context(&mut self, egl_context: EGLContext) -> GlobalContext {
        let mut selected_context = None;
        for (i, context) in self.contexts.iter().enumerate() {
            if context as *const GlobalContext as EGLContext == egl_context {
                selected_context = Some(i);
            }
        }

        match selected_context {
            Some(id_context) => self.contexts.remove(id_context),
            None => panic!("Can't get context"),
        }
    }

    pub fn is_surface(&self, egl_surface: EGLSurface) -> bool {
        for surface in self.surfaces.iter() {
            if surface as *const GlobalSurface as EGLSurface == egl_surface {
                return true;
            }
        }
        false
    }

    pub fn drain_surface(&mut self, egl_surface: EGLSurface) -> GlobalSurface {
        let mut selected_surface = None;
        for (i, surface) in self.surfaces.iter().enumerate() {
            if surface as *const GlobalSurface as EGLSurface == egl_surface {
                selected_surface = Some(i);
            }
        }

        match selected_surface {
            Some(id_surface) => self.surfaces.remove(id_surface),
            None => panic!("Can't get surface"),
        }
    }
}

// Contains the pointer passed with getDisplay
pub struct PlatformDisplay {
    pub display_id: Arc<Unique<c_void>>,
}

impl PlatformDisplay {
    pub fn new(display_id: *mut c_void) -> PlatformDisplay {
        PlatformDisplay {
            display_id: Arc::new(
                Unique::new(display_id).expect("You must pass a valid pointer for display"),
            ),
        }
    }
}