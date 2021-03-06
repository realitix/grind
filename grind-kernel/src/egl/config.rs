use egl::types::EGLint;

pub struct Config {
    pub alpha_size: i32,
    pub alpha_mask_size: i32,
    pub bind_to_texture_rgb: i32,
    pub bind_to_texture_rgba: i32,
    pub blue_size: i32,
    pub buffer_size: i32,
    pub color_buffer_type: i32,
    pub config_caveat: i32,
    pub config_id: i32,
    pub conformant: i32,
    pub depth_size: i32,
    pub green_size: i32,
    pub level: i32,
    pub luminance_size: i32,
    pub max_pbuffer_width: i32,
    pub max_pbuffer_height: i32,
    pub max_pbuffer_pixels: i32,
    pub max_swap_interval: i32,
    pub min_swap_interval: i32,
    pub native_renderable: i32,
    pub native_visual_id: i32,
    pub native_visual_type: i32,
    pub red_size: i32,
    pub renderable_type: i32,
    pub sample_buffers: i32,
    pub samples: i32,
    pub stencil_size: i32,
    pub surface_type: i32,
    pub transparent_type: i32,
    pub transparent_red_value: i32,
    pub transparent_green_value: i32,
    pub transparent_blue_value: i32,
}

impl Config {
    pub fn get_attrib(&self, attribute: EGLint) -> Option<EGLint> {
        Some(2)
    }
}
