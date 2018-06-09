use std::sync::Arc;
use std::vec::IntoIter;

use vulkano::buffer::cpu_pool::CpuBufferPoolChunk;
use vulkano::buffer::BufferAccess;
use vulkano::buffer::CpuBufferPool;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::memory::pool::StdMemoryPool;
use vulkano::pipeline::shader::ShaderInterfaceDef;
use vulkano::pipeline::vertex::AttributeInfo;
use vulkano::pipeline::vertex::IncompatibleVertexDefinitionError;
use vulkano::pipeline::vertex::InputRate;
use vulkano::pipeline::vertex::Vertex;
use vulkano::pipeline::vertex::VertexDefinition;
use vulkano::pipeline::vertex::VertexMemberInfo;
use vulkano::pipeline::vertex::VertexMemberTy;
use vulkano::pipeline::vertex::VertexSource;
use vulkano::SafeDeref;

// IMPL Custom Buffer Definition
pub struct GrindBufferDefinition;

unsafe impl<I> VertexDefinition<I> for GrindBufferDefinition
where
    I: ShaderInterfaceDef,
{
    type BuffersIter = IntoIter<(u32, usize, InputRate)>;
    type AttribsIter = IntoIter<(u32, u32, AttributeInfo)>;

    fn definition(
        &self,
        interface: &I,
    ) -> Result<(Self::BuffersIter, Self::AttribsIter), IncompatibleVertexDefinitionError> {
        let buffers = vec![(
            0 as u32,    // binding
            12 as usize, // vertex size (3xfloat)
            InputRate::Vertex,
        )];
        //let attribs = vec![(0, 0, AttributeInfo{offset: 0, format: Format::R8G8B8Unorm})];
        let attribs = vec![(
            0,
            0,
            AttributeInfo {
                offset: 0,
                format: Format::R32G32B32Sfloat,
            },
        )];

        Ok((buffers.into_iter(), attribs.into_iter()))
    }
}

unsafe impl VertexSource<CpuBufferPoolChunk<u8, Arc<StdMemoryPool>>> for GrindBufferDefinition {
    fn decode(
        &self,
        mut source: CpuBufferPoolChunk<u8, Arc<StdMemoryPool>>,
    ) -> (Vec<Box<BufferAccess + Send + Sync>>, usize, usize) {
        let mut len = source.size() / 4; // float
        len = len / 3; // vec3
        (vec![Box::new(source)], len, 1)
    }
}

unsafe impl<T> VertexSource<Vec<T>> for GrindBufferDefinition {
    fn decode(&self, _: Vec<T>) -> (Vec<Box<BufferAccess + Sync + Send + 'static>>, usize, usize) {
        panic!("bufferless drawing should not be supplied with buffers")
    }
}

// END IMPL

pub struct Buffer {
    inner: CpuBufferPool<u8>,
    pub chunk: Option<CpuBufferPoolChunk<u8, Arc<StdMemoryPool>>>,
}

impl Buffer {
    pub fn new(device: Arc<Device>) -> Buffer {
        Buffer {
            inner: CpuBufferPool::vertex_buffer(device),
            chunk: None,
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.chunk = Some(self.inner.chunk(data.iter().cloned()).unwrap());
    }
}
