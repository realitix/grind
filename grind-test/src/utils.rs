use std::path::Path;
use opengles2 as gl;


pub struct ShaderProgram {
    pub program: gl::Uint
}

impl ShaderProgram {
    pub fn new(vertex_source: &str, fragment_source: &str) -> ShaderProgram {
        let program = gl::create_program();

        //let vertex_source = fs::read_to_string(vertex_path).unwrap();
        //let fragment_source = fs::read_to_string(fragment_path).unwrap();

        let vertex_shader = ShaderProgram::get_shader(&vertex_source, gl::VERTEX_SHADER);
        let fragment_shader = ShaderProgram::get_shader(&fragment_source, gl::FRAGMENT_SHADER);

        gl::attach_shader(program, vertex_shader);
        gl::attach_shader(program, fragment_shader);
        gl::link_program(program);

        if gl::get_programiv(program, gl::LINK_STATUS) != gl::TRUE as i32 {
            let info = gl::get_program_info_log(program);
            gl::delete_program(program);
            gl::delete_shader(vertex_shader);
            gl::delete_shader(fragment_shader);
            panic!("Program compilation failed: \n {}", info);
        }

        gl::delete_shader(vertex_shader);
        gl::delete_shader(fragment_shader);

        ShaderProgram { program }
    }

    fn get_shader(source: &str, shader_type: gl::Enum) -> gl::Uint {
        let shader = gl::create_shader(shader_type);
        gl::shader_source(shader, source);
        gl::compile_shader(shader);

        if gl::get_shaderiv(shader, gl::COMPILE_STATUS) != gl::TRUE as i32 {
            let info = gl::get_shader_info_log(shader);
            gl::delete_shader(shader);
            panic!("Shader compilation failed: \n {}", info);
        }

        shader
    }

    pub fn get_attrib_location(&self, name: &str) -> i32 {
        gl::get_attrib_location(self.program, name)
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        gl::get_uniform_location(self.program, name)
    }
}

pub fn assert_gl(width: i32, height: i32, expected: &'static [u8]) {
    let observed_raw = gl::read_pixels(0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE);
    // convert the observed_raw to png to compare properly the result
    let mut observed: Vec<u8> = Vec::new();
    image::png::PNGEncoder::new(&mut observed)
        .encode(&observed_raw, width as u32, height as u32, image::RGBA(8))
        .unwrap();
        
    assert!(&expected[..] == &observed[..], "Observed is not equal to expected!");
}

pub fn write_gl_buffer(width: i32, height: i32) {
    let observed_raw = gl::read_pixels(0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE);
    image::save_buffer(
        &Path::new("test_image.png"),
        &observed_raw,
        width as u32,
        height as u32,
        image::RGBA(8),
    ).unwrap();
}