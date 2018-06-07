use std::borrow::Cow::Borrowed;
use std::ffi::CStr;
use std::sync::Arc;
use vulkano::descriptor::descriptor::DescriptorDesc;
use vulkano::descriptor::descriptor::ShaderStages;
use vulkano::descriptor::pipeline_layout::PipelineLayoutDesc;
use vulkano::descriptor::pipeline_layout::PipelineLayoutDescPcRange;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::pipeline::shader::GraphicsEntryPoint;
use vulkano::pipeline::shader::GraphicsShaderType;
use vulkano::pipeline::shader::ShaderInterfaceDef;
use vulkano::pipeline::shader::ShaderInterfaceDefEntry;
use vulkano::pipeline::shader::ShaderModule;
use vulkano::pipeline::shader::SpecializationConstants;
use vulkano::pipeline::shader::SpecializationMapEntry;

// **********
// Main Input
// **********
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MainInput;

unsafe impl ShaderInterfaceDef for MainInput {
    type Iter = MainInputIter;
    fn elements(&self) -> MainInputIter {
        MainInputIter { num: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MainInputIter {
    num: u16,
}
impl Iterator for MainInputIter {
    type Item = ShaderInterfaceDefEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            self.num += 1;

            return Some(ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R8G8B8Unorm,
                name: Some(Borrowed("vin_position")),
            });
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (1 - self.num) as usize;
        (len, Some(len))
    }
}

impl ExactSizeIterator for MainInputIter {}

// **********
// Main Output
// **********
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MainOutput;

unsafe impl ShaderInterfaceDef for MainOutput {
    type Iter = MainOutputIter;
    fn elements(&self) -> MainOutputIter {
        MainOutputIter { num: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MainOutputIter {
    num: u16,
}
impl Iterator for MainOutputIter {
    type Item = ShaderInterfaceDefEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            self.num += 1;

            return Some(ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Borrowed("f_color")),
            });
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (1 - self.num) as usize;
        (len, Some(len))
    }
}

impl ExactSizeIterator for MainOutputIter {}

// **********
// Push Constant (Empty)
// **********
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct EmptySpecializationConstants {}

impl Default for EmptySpecializationConstants {
    fn default() -> EmptySpecializationConstants {
        EmptySpecializationConstants {}
    }
}

unsafe impl SpecializationConstants for EmptySpecializationConstants {
    fn descriptors() -> &'static [SpecializationMapEntry] {
        static DESCRIPTORS: [SpecializationMapEntry; 0] = [];
        &DESCRIPTORS
    }
}

// **********
// Layout
// **********
#[derive(Debug, Clone)]
pub struct Layout(pub ShaderStages);

unsafe impl PipelineLayoutDesc for Layout {
    fn num_sets(&self) -> usize {
        0
    }

    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        match set {
            _ => None,
        }
    }

    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
        match (set, binding) {
            _ => None,
        }
    }

    fn num_push_constants_ranges(&self) -> usize {
        0
    }

    fn push_constants_range(&self, num: usize) -> Option<PipelineLayoutDescPcRange> {
        if num != 0 || 0 == 0 {
            return None;
        }
        Some(PipelineLayoutDescPcRange {
            offset: 0, // FIXME: not necessarily true
            size: 0,
            stages: ShaderStages::all(), // FIXME: wrong
        })
    }
}

// **********
// Shader
// **********
pub struct Shader {
    module: Arc<ShaderModule>,
    shader_type: GraphicsShaderType,
}

impl Shader {
    pub fn new(device: Arc<Device>, spirv: &[u8], shader_type: GraphicsShaderType) -> Shader {
        let module = unsafe { ShaderModule::new(device, spirv).unwrap() };
        Shader {
            module,
            shader_type,
        }
    }

    pub fn main_entry_point(
        &self,
    ) -> GraphicsEntryPoint<EmptySpecializationConstants, MainInput, MainOutput, Layout> {
        unsafe {
            static NAME: [u8; 5] = [109, 97, 105, 110, 0]; // "main"
            let layout = match self.shader_type {
                GraphicsShaderType::Vertex => Layout(ShaderStages {
                    vertex: true,
                    ..ShaderStages::none()
                }),
                GraphicsShaderType::Fragment => Layout(ShaderStages {
                    fragment: true,
                    ..ShaderStages::none()
                }),
                _ => panic!("Unknow shader type")
            };

            self.module.graphics_entry_point(
                CStr::from_ptr(NAME.as_ptr() as *const _),
                MainInput,
                MainOutput,
                layout,
                self.shader_type,
            )
        }
    }
}
