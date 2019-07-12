mod buffer;
mod shader;
mod util;

use std::collections::HashMap;
use std::sync::Arc;
use std::vec::Vec;

use kernel::vulkan::buffer::VertexAttributes;
use kernel::vulkan::VulkanDriver;
use kernel::vulkan::vulkanobject as vo;

use opengl::gles2::buffer::Buffer;
use opengl::gles2::shader::{Shader, ShaderProgram};
use opengl::gles2::util::{get_stride, get_vk_format};
use opengl::types::*;

pub struct ContextGlES2 {
    kernel: VulkanDriver,
    clear_color: [GLclampf; 4],
    vertex_attributes: Arc<VertexAttributes>,

    // program attributes
    programs: Vec<ShaderProgram>,
    program_binded: GLuint,

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
            vertex_attributes: Arc::new(VertexAttributes::new()),
            programs: Vec::new(),
            program_binded: 0,
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

        current_program.unwrap().link(&self.kernel, &self.shaders);
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
            .buffer_data(&self.kernel, target, size, data, usage);
    }

    pub fn enable_vertex_attrib_array(&mut self, index: GLuint) {
        Arc::get_mut(&mut self.vertex_attributes)
            .unwrap()
            .enable_attribute(index);
    }

    pub fn get_attrib_location(&mut self, program_id: GLuint, name: *const GLchar) -> GLint {
        // Get program
        let mut current_program = None;
        for program in self.programs.iter_mut() {
            if program.id == program_id {
                current_program = Some(program);
            }
        }

        current_program.unwrap().get_attrib_location(name)
    }

    pub fn vertex_attrib_pointer(
        &mut self,
        index: GLuint,
        size: GLint,
        _type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        ptr: *const GLvoid,
    ) {
        let vk_stride = match stride {
            0 => util::get_stride(size, _type),
            x => x as u32,
        };

        let offset = ptr as *const _ as u32;

        Arc::get_mut(&mut self.vertex_attributes)
            .unwrap()
            .set_attribute(
                index,
                self.buffer_binded,
                get_vk_format(size, _type),
                vk_stride,
                offset,
            );
    }

    pub fn use_program(&mut self, program_id: GLuint) {
        // Get program
        let mut current_program = None;
        for program in self.programs.iter() {
            if program.id == program_id {
                current_program = Some(program);
            }
        }

        match current_program {
            Some(p) => self.program_binded = program_id,
            None => self.program_binded = 0, // unbind program
        };
    }

    pub fn draw_arrays(&mut self, mode: GLenum, first: GLint, count: GLsizei) {
        // Get program
        let mut current_program = None;
        for program in self.programs.iter() {
            if program.id == self.program_binded {
                current_program = Some(program);
            }
        }

        // Get buffer
        let mut current_buffer = None;
        for buffer in self.buffers.iter() {
            if buffer.id == self.buffer_binded {
                current_buffer = Some(buffer);
            }
        }

        // Collect all VK Buffers
        let mut vk_buffers = HashMap::new();
        for buffer in self.buffers.iter() {
            vk_buffers.insert(buffer.id, buffer.get_buffer());
        }

        let vs = current_program.unwrap().get_vertex_shader();
        let fs = current_program.unwrap().get_fragment_shader();

        self.kernel
            .draw(vs, fs, vk_buffers, self.vertex_attributes.clone());
    }

    pub fn read_pixels(
        &mut self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        _type: GLenum,
        pixels: *mut GLvoid,
    ) {
        let format = util::get_type_vk_format(format, _type);
        self.kernel.read_pixels(x, y, width, height, format, pixels);
    }
}
