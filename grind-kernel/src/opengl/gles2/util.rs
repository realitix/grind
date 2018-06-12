use opengl::types::*;
use std::collections::HashMap;
use vulkano::format::Format;

lazy_static! {
    static ref FORMAT_VK_MAPPING: HashMap<(GLint, GLenum), Format> = {
        let mut m = HashMap::new();
        m.insert((1, FLOAT), Format::R32Sfloat);
        m.insert((2, FLOAT), Format::R32G32Sfloat);
        m.insert((3, FLOAT), Format::R32G32B32Sfloat);
        m.insert((4, FLOAT), Format::R32G32B32A32Sfloat);
        // TODO complete format with GL_BYTE, GL_UNSIGNED_BYTE, GL_SHORT, GL_UNSIGNED_SHORT, GL_FIXED
        m
    };

    static ref FORMAT_SIZE_MAPPING: HashMap<GLenum, usize> = {
        let mut m = HashMap::new();
        m.insert(FLOAT, 4);
        // TODO complete format with GL_BYTE, GL_UNSIGNED_BYTE, GL_SHORT, GL_UNSIGNED_SHORT, GL_FIXED
        m
    };
}

pub fn get_vk_format(size: GLint, _type: GLenum) -> Format {
    *FORMAT_VK_MAPPING
        .get(&(size, _type))
        .expect("Unknow format")
}

pub fn get_stride(size: GLint, _type: GLenum) -> usize {
    let format_size = *FORMAT_SIZE_MAPPING.get(&_type).expect("Unknow format");
    size as usize * format_size
}
