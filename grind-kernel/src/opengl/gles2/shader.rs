use std::ffi::CStr;
use std::io::Read;
use std::mem;

use glsltranspiler::{transpile, ShaderType};
use shaderc::{CompilationArtifact, CompileOptions, Compiler, ShaderKind};

use opengl::types::*;

struct ReflectResult {}

fn reflect<R>(mut spirv: R) -> ReflectResult
where
    R: Read,
{
    let mut data = Vec::new();
    spirv.read_to_end(&mut data);
    //let doc = parse::parse_spirv(&data);
    ReflectResult {}
}

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
        let shader_type;
        let shader_kind;
        let shader_name;

        match self.shader_type {
            VERTEX_SHADER => {
                shader_type = ShaderType::Vertex;
                shader_kind = ShaderKind::Vertex;
                shader_name = String::from("Vertex");
            }
            FRAGMENT_SHADER => {
                shader_type = ShaderType::Fragment;
                shader_kind = ShaderKind::Fragment;
                shader_name = String::from("Fragment");
            }
            _ => panic!("Unknow shader type"),
        };

        // TODO: 1. check shader validity
        // 2. transpile shader to version 450
        let transpilation = transpile(&self.source, shader_type);
        self.source_transpiled = Some(transpilation.text);

        // 3. compile to spirv
        //println!("{}", self.source_transpiled.as_ref().unwrap());
        let mut compiler = Compiler::new().unwrap();
        let mut options = CompileOptions::new().unwrap();
        self.spirv = Some(
            compiler
                .compile_into_spirv(
                    self.source_transpiled.as_ref().unwrap(),
                    shader_kind,
                    &shader_name,
                    "main",
                    Some(&options),
                )
                .unwrap(),
        );
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
    vertex: Option<Shader>,
    fragment: Option<Shader>,
    linked: bool,
}

impl ShaderProgram {
    pub fn new(id: GLuint) -> ShaderProgram {
        ShaderProgram {
            id: id,
            vertex: None,
            fragment: None,
            linked: false,
        }
    }

    pub fn attach(&mut self, shader: Shader) {
        let t = shader.shader_type;
        match t {
            VERTEX_SHADER => self.vertex = Some(shader),
            FRAGMENT_SHADER => self.fragment = Some(shader),
            _ => {}
        };
    }

    pub fn link(&mut self) {
        // TODO: Check linking with glslang
        self.linked = true;
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
}
