mod shader;

use std::sync::Arc;
use std::vec::Vec;

use kernel::vulkan::VulkanDriver;
use opengl::gles2::shader::{Shader, ShaderProgram};
use opengl::types::*;

pub struct ContextGlES2 {
    kernel: VulkanDriver,
    clear_color: [GLclampf; 4],
    programs: Vec<ShaderProgram>,
    shaders: Vec<Shader>,
    shaders_id: GLuint,
}

impl ContextGlES2 {
    pub fn new(kernel: VulkanDriver) -> ContextGlES2 {
        ContextGlES2 {
            kernel,
            clear_color: [0.; 4],
            programs: Vec::new(),
            shaders: Vec::new(),
            shaders_id: 0,
        }
    }

    pub fn clear_color(&mut self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        self.clear_color = [red, green, blue, alpha]
    }

    pub fn clear(&mut self, mask: GLbitfield) {
        self.kernel.clear(self.clear_color);
    }

    pub fn swap_buffers(&mut self) {
        self.kernel.present();
    }

    pub fn create_program(&mut self) -> GLuint {
        let id = (self.programs.len() + 1) as GLuint;
        let program = ShaderProgram::new(id);
        self.programs.push(program);
        id
    }

    pub fn create_shader(&mut self, shader_type: GLenum) -> GLuint {
        self.shaders_id += 1;
        let id = self.shaders_id;
        let shader = Shader::new(id, shader_type);
        self.shaders.push(shader);
        id
    }

    pub fn shader_source(
        &mut self,
        shader_id: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        // Retrieve shader
        let mut current_shader = None;
        for shader in self.shaders.iter_mut() {
            if shader.id == shader_id {
                current_shader = Some(shader);
            }
        }

        current_shader.unwrap().set_source(count, string, length)
    }

    pub fn compile_shader(&mut self, shader_id: GLuint) {
        let mut current_shader = None;
        for shader in self.shaders.iter_mut() {
            if shader.id == shader_id {
                current_shader = Some(shader);
            }
        }

        current_shader.unwrap().compile();
    }
}
