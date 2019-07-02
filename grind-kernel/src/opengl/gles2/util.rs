use opengl::types as gl;
use std::collections::HashMap;
use kernel::vulkan::vulkanobject as vo;

lazy_static! {
    static ref FORMAT_VK_MAPPING: HashMap<(gl::GLint, gl::GLenum), vo::Format> = {
        let mut m = HashMap::new();
        m.insert((1, gl::FLOAT), vo::Format::R32_SFLOAT);
        m.insert((2, gl::FLOAT), vo::Format::R32G32_SFLOAT);
        m.insert((3, gl::FLOAT), vo::Format::R32G32B32_SFLOAT);
        m.insert((4, gl::FLOAT), vo::Format::R32G32B32A32_SFLOAT);
        // TODO complete format with GL_BYTE, GL_UNSIGNED_BYTE, GL_SHORT, GL_UNSIGNED_SHORT, GL_FIXED
        m
    };

    // Used for glReadPixels
    static ref FORMAT_TYPE_VK_MAPPING: HashMap<(gl::GLenum, gl::GLenum), vo::Format> = {
        let mut m = HashMap::new();
        m.insert((gl::RGBA, gl::UNSIGNED_BYTE), vo::Format::R8G8B8A8_UNORM);
        // TODO complete format with GL_BYTE, GL_UNSIGNED_BYTE, GL_SHORT, GL_UNSIGNED_SHORT, GL_FIXED
        m
    };

    static ref FORMAT_SIZE_MAPPING: HashMap<gl::GLenum, u32> = {
        let mut m = HashMap::new();
        m.insert(gl::FLOAT, 4);
        // TODO complete format with GL_BYTE, GL_UNSIGNED_BYTE, GL_SHORT, GL_UNSIGNED_SHORT, GL_FIXED
        m
    };
}

pub fn get_vk_format(size: gl::GLint, _type: gl::GLenum) -> vo::Format {
    *FORMAT_VK_MAPPING
        .get(&(size, _type))
        .expect("Unknow format")
}

pub fn get_type_vk_format(format: gl::GLenum, _type: gl::GLenum) -> vo::Format {
    *FORMAT_TYPE_VK_MAPPING
        .get(&(format, _type))
        .expect("Unknow format")
}

pub fn get_stride(size: gl::GLint, _type: gl::GLenum) -> u32 {
    let format_size = *FORMAT_SIZE_MAPPING.get(&_type).expect("Unknow format");
    size as u32 * format_size
}
