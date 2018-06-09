use std::ffi::CStr;
use std::io::Read;
use std::mem;
use std::sync::Arc;

use glsltranspiler::{transpile, ShaderType};
use shaderc::{CompilationArtifact, CompileOptions, Compiler, ShaderKind};
use spirvparser::{reflect, SpirvReflection};
use vulkano::pipeline::shader::GraphicsShaderType;

use kernel::vulkan::shader::Shader as VulkanShader;
use kernel::vulkan::VulkanDriver;
use opengl::types::*;

pub struct Shader {
    pub id: GLuint,
    pub shader_type: GLenum,
    source: String,
    source_transpiled: Option<String>,
    spirv: Option<CompilationArtifact>,
}

impl Shader {
    pub fn new(id: GLuint, shader_type: GLenum) -> Shader {
        Shader {
            id,
            shader_type,
            source: String::new(),
            source_transpiled: None,
            spirv: None,
        }
    }

    pub fn set_source(
        &mut self,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ) {
        let mut result = String::new();
        for i in 0..count {
            let slice = unsafe { CStr::from_ptr(*string.offset(i as isize)) };
            let clean_slice = slice.to_str().unwrap();
            result.push_str(clean_slice);
            result.push('\n');
        }

        mem::replace(&mut self.source, result);
    }

    pub fn compile(&mut self) {
        let shader_type = match self.shader_type {
            VERTEX_SHADER => ShaderType::Vertex,
            FRAGMENT_SHADER => ShaderType::Fragment,
            _ => panic!("Unknow shader type"),
        };

        // TODO: 1. check shader validity
        // 2. transpile shader to version 450
        let transpilation = transpile(&self.source, shader_type);
        self.source_transpiled = Some(transpilation.text);
    }

    pub fn get_shaderiv(&self, pname: GLenum, params: *mut GLint) {
        match pname {
            GL_COMPILE_STATUS => match self.source_transpiled {
                Some(ref x) => unsafe { *params = TRUE as i32 },
                None => unsafe { *params = FALSE as i32 },
            },
            _ => {}
        };
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
    vertex_id: Option<GLuint>,
    fragment_id: Option<GLuint>,
    linked: bool,
    vertex_shader: Option<Arc<VulkanShader>>,
    vertex_reflection: Option<SpirvReflection>,
    fragment_shader: Option<Arc<VulkanShader>>,
    fragment_reflection: Option<SpirvReflection>,
}

impl ShaderProgram {
    pub fn new(id: GLuint) -> ShaderProgram {
        ShaderProgram {
            id: id,
            vertex_id: None,
            fragment_id: None,
            linked: false,
            vertex_shader: None,
            fragment_shader: None,
            vertex_reflection: None,
            fragment_reflection: None,
        }
    }

    pub fn attach(&mut self, shader: &Shader) {
        let t = shader.shader_type;
        match t {
            VERTEX_SHADER => self.vertex_id = Some(shader.id),
            FRAGMENT_SHADER => self.fragment_id = Some(shader.id),
            _ => {}
        };
    }

    pub fn link(&mut self, kernel: &VulkanDriver, shaders: &Vec<Shader>) {
        // 1. TODO: Check linking with glslang
        // 2. Find Vertex and Fragment shaders
        let mut vertex_shader = None;
        let mut fragment_shader = None;
        for shader in shaders.iter() {
            if shader.id == self.vertex_id.unwrap() {
                vertex_shader = Some(shader);
            }
            if shader.id == self.fragment_id.unwrap() {
                fragment_shader = Some(shader);
            }
        }

        // 2. Compile to spirv
        let mut compiler = Compiler::new().unwrap();
        let mut options = CompileOptions::new().unwrap();

        // vertex
        let vertex_spirv = Some(
            compiler
                .compile_into_spirv(
                    vertex_shader.unwrap().source_transpiled.as_ref().unwrap(),
                    ShaderKind::Vertex,
                    &String::from("Vertex"),
                    "main",
                    Some(&options),
                )
                .unwrap(),
        );
        self.vertex_shader = Some(Arc::new(kernel.new_shader(
            vertex_spirv.as_ref().unwrap().as_binary_u8(),
            GraphicsShaderType::Vertex,
        )));
        self.vertex_reflection = Some(reflect(vertex_spirv.as_ref().unwrap().as_binary_u8()));

        // fragment
        let fragment_spirv = Some(
            compiler
                .compile_into_spirv(
                    fragment_shader.unwrap().source_transpiled.as_ref().unwrap(),
                    ShaderKind::Fragment,
                    &String::from("Fragment"),
                    "main",
                    Some(&options),
                )
                .unwrap(),
        );
        self.fragment_shader = Some(Arc::new(kernel.new_shader(
            fragment_spirv.as_ref().unwrap().as_binary_u8(),
            GraphicsShaderType::Fragment,
        )));
        self.fragment_reflection = Some(reflect(fragment_spirv.as_ref().unwrap().as_binary_u8()));

        self.linked = true;
    }

    pub fn get_vertex_shader(&self) -> Arc<VulkanShader> {
        self.vertex_shader.as_ref().unwrap().clone()
    }

    pub fn get_fragment_shader(&self) -> Arc<VulkanShader> {
        self.fragment_shader.as_ref().unwrap().clone()
    }

    pub fn get_programiv(&self, pname: GLenum, params: *mut GLint) {
        match pname {
            GL_LINK_STATUS => match self.linked {
                true => unsafe { *params = TRUE as i32 },
                false => unsafe { *params = FALSE as i32 },
            },
            _ => {}
        };
    }

    pub fn get_attrib_location(&self, name_ptr: *const GLchar) -> GLint {
        let cname = unsafe { CStr::from_ptr(name_ptr) };
        let name = cname.to_str().unwrap();

        *self.vertex_reflection
            .as_ref()
            .unwrap()
            .input
            .get(name)
            .unwrap() as GLint
    }

    pub fn vertex_attrib_pointer(
        index: GLuint,
        size: GLint,
        _type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        ptr: *const GLvoid,
    ) {
    }
}
