use kernel::Kernel;
use kernel::vulkan::VulkanDriver;
use kernel::vulkan::is_available as vulkan_is_available;

use egl::wayland::WaylandDisplay;
use egl::config::Config;
use egl::types::*;


pub fn is_available() -> bool {
    vulkan_is_available()
}

pub struct Display {
    native_display: WaylandDisplay,
    kernel: VulkanDriver,
    pub configs: Vec<Config>
}

impl Display {
    pub fn new(native_display: WaylandDisplay, kernel: VulkanDriver) -> Display {
        Display {
            native_display,
            kernel,
            configs: Vec::new()
        }
    }

    pub fn initialize(&mut self) {
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
            transparent_blue_value: 0
        });
    }
}
