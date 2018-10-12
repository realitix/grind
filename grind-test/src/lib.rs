use std::fs;

extern crate opengles2 as gl;

pub struct ShaderProgram {
    pub program: gl::Uint
}

impl ShaderProgram {
    pub fn new(vertex_path: &str, fragment_path: &str) -> ShaderProgram {
        let program = gl::create_program();

        let vertex_source = fs::read_to_string(vertex_path).unwrap();
        let fragment_source = fs::read_to_string(fragment_path).unwrap();

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
