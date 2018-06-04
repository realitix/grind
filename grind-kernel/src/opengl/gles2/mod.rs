mod buffer;
mod shader;

use std::sync::Arc;
use std::vec::Vec;

use kernel::vulkan::VulkanDriver;
use opengl::gles2::buffer::Buffer;
use opengl::gles2::shader::{Shader, ShaderProgram};
use opengl::types::*;

pub struct ContextGlES2 {
    kernel: VulkanDriver,
    clear_color: [GLclampf; 4],
    programs: Vec<ShaderProgram>,

    // shader attributes
    shaders: Vec<Shader>,
    next_shader_id: GLuint,

    // buffer attributes
    buffers: Vec<Buffer>,
    next_buffer_id: GLuint,
    buffer_binded: GLuint,
}

impl ContextGlES2 {
    pub fn new(kernel: VulkanDriver) -> ContextGlES2 {
        ContextGlES2 {
            kernel,
            clear_color: [0.; 4],
            programs: Vec::new(),
            shaders: Vec::new(),
            next_shader_id: 0,
            buffers: Vec::new(),
            next_buffer_id: 0,
            buffer_binded: 0,
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
        self.next_shader_id += 1;
        let id = self.next_shader_id;
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
        // TODO: Refactor get shader
        let mut current_shader = None;
        for shader in self.shaders.iter_mut() {
            if shader.id == shader_id {
                current_shader = Some(shader);
            }
        }

        current_shader.unwrap().compile();
    }

    pub fn get_shaderiv(&self, shader_id: GLuint, pname: GLenum, params: *mut GLint) {
        // TODO: Refactor get shader
        let mut current_shader = None;
        for shader in self.shaders.iter() {
            if shader.id == shader_id {
                current_shader = Some(shader);
            }
        }

        current_shader.unwrap().get_shaderiv(pname, params);
    }

    pub fn attach_shader(&mut self, program_id: GLuint, shader_id: GLuint) {
        // TODO: Refactor get shader
        let mut current_shader = None;
        for shader in self.shaders.iter() {
            if shader.id == shader_id {
                current_shader = Some(shader);
            }
        }

        // Get Program
        let mut current_program = None;
        for program in self.programs.iter_mut() {
            if program.id == program_id {
                current_program = Some(program);
            }
        }

        current_program.unwrap().attach(current_shader.unwrap());
    }

    pub fn link_program(&mut self, program_id: GLuint) {
        // Get Program
        let mut current_program = None;
        for program in self.programs.iter_mut() {
            if program.id == program_id {
                current_program = Some(program);
            }
        }

        current_program.unwrap().link(&self.shaders);
    }

    pub fn get_programiv(&self, program_id: GLuint, pname: GLenum, params: *mut GLint) {
        // Get Program
        let mut current_program = None;
        for program in self.programs.iter() {
            if program.id == program_id {
                current_program = Some(program);
            }
        }

        current_program.unwrap().get_programiv(pname, params);
    }

    pub fn delete_shader(&mut self, shader_id: GLuint) {
        self.shaders.retain(|ref shader| shader.id != shader_id);
    }

    pub fn gen_buffers(&mut self, n: GLsizei, buffers: *mut GLuint) {
        for i in 0..n {
            self.next_buffer_id += 1;
            let id = self.next_buffer_id;
            let buffer = Buffer::new(id, &self.kernel);
            self.buffers.push(buffer);

            unsafe {
                *buffers.offset(i as isize) = id;
            }
        }
    }

    pub fn bind_buffer(&mut self, target: GLenum, buffer_id: GLuint) {
        // Get buffer
        let mut current_buffer = None;
        for buffer in self.buffers.iter_mut() {
            if buffer.id == buffer_id {
                current_buffer = Some(buffer);
            }
        }

        match current_buffer {
            Some(b) => {
                b.target = target;
                self.buffer_binded = buffer_id;
            }
            None => self.buffer_binded = 0, // unbind buffer
        };
    }

    pub fn buffer_data(
        &mut self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const GLvoid,
        usage: GLenum,
    ) {
        // Get buffer
        let mut current_buffer = None;
        for buffer in self.buffers.iter_mut() {
            if buffer.id == self.buffer_binded {
                current_buffer = Some(buffer);
            }
        }

        current_buffer
            .unwrap()
            .buffer_data(target, size, data, usage);
    }

    pub fn enable_vertex_attrib_array(&mut self, index: GLuint) {
        // Get buffer
        let mut current_buffer = None;
        for buffer in self.buffers.iter_mut() {
            if buffer.id == self.buffer_binded {
                current_buffer = Some(buffer);
            }
        }

        current_buffer.unwrap().enable_vertex_attrib_array(index);
    }
}
