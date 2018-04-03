use kernel::Kernel;
use kernel::vulkan::is_available as vulkan_is_available;

use egl::wayland::WaylandDisplay;
use egl::config::Config;
use egl::types::*;
use egl::global::EGL_RESULT;


pub fn is_available() -> bool {
    vulkan_is_available()
}

pub struct Display {
    pub native_display: WaylandDisplay,
    pub configs: Vec<Config>,
}

impl Display {
    pub fn new(native_display: WaylandDisplay) -> Display {
        Display {
            native_display,
            configs: Vec::new(),
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

    pub fn with_config<F>(&self, egl_config: EGLConfig, f: F) -> EGLBoolean
        where F: FnOnce(&Config) -> EGLBoolean
    {
        let mut current_config: Option<&Config> = None;

        for config in self.configs.iter() {
            if config as *const Config as EGLConfig == egl_config {
                current_config = Some(config);
            }
        }

        match current_config {
            None => EGL_RESULT(EGL_BAD_CONFIG),
            Some(c) => f(c)
        }
    }
}
