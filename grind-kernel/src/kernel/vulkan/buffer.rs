use std::collections::HashMap;
use std::mem;
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

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub enabled: bool,
    pub buffer_id: u32,
    pub format: Format,
    pub stride: usize,
    pub offset: usize,
}

impl VertexAttribute {
    pub fn new() -> VertexAttribute {
        VertexAttribute {
            enabled: false,
            buffer_id: 0,
            format: Format::R8Unorm,
            stride: 0,
            offset: 0,
        }
    }

    pub fn update(&mut self, buffer_id: u32, format: Format, stride: usize, offset: usize) {
        self.buffer_id = buffer_id;
        self.format = format;
        self.stride = stride;
        self.offset = offset;
    }
}

#[derive(Clone)]
pub struct VertexAttributes {
    // key = index (location)
    pub attributes: HashMap<u32, VertexAttribute>,
    // key: buffer_id, val: buffer_binding
    pub buffers_binding: HashMap<u32, u32>,
    // key: buffer_binding, val: buffer_id
    pub binding_buffers: HashMap<u32, u32>,
}

impl VertexAttributes {
    pub fn new() -> VertexAttributes {
        VertexAttributes {
            attributes: HashMap::new(),
            buffers_binding: HashMap::new(),
            binding_buffers: HashMap::new(),
        }
    }

    pub fn check_index(&mut self, index: u32) {
        let index_exists = {
            match self.attributes.get(&index) {
                None => false,
                Some(x) => true,
            }
        };

        if !index_exists {
            self.attributes.insert(index, VertexAttribute::new());
        }
    }

    pub fn set_attribute(
        &mut self,
        index: u32,
        buffer_id: u32,
        format: Format,
        stride: usize,
        offset: usize,
    ) {
        self.check_index(index);
        self.attributes
            .get_mut(&index)
            .unwrap()
            .update(buffer_id, format, stride, offset);
    }

    pub fn enable_attribute(&mut self, index: u32) {
        self.check_index(index);
        self.attributes.get_mut(&index).unwrap().enabled = true;
    }

    pub fn disable_attribute(&mut self, index: u32) {
        self.check_index(index);
        self.attributes.get_mut(&index).unwrap().enabled = false;
    }

    pub fn generate_buffers_binding_map(&mut self) {
        let mut buffers_binding = HashMap::new();
        let mut binding_buffers = HashMap::new();
        let mut buffer_cache = Vec::new();
        let mut binding = 0;

        for (index, attribute) in self.attributes.iter() {
            if buffer_cache.contains(&attribute.buffer_id) || !attribute.enabled {
                continue;
            }

            buffers_binding.insert(attribute.buffer_id, binding);
            binding_buffers.insert(binding, attribute.buffer_id);
            buffer_cache.push(attribute.buffer_id);
            binding += 1;
        }

        mem::replace(&mut self.buffers_binding, buffers_binding);
        mem::replace(&mut self.binding_buffers, binding_buffers);
    }

    // return (binding, stride, inputrate)
    pub fn get_buffers_definition(&self) -> Vec<(u32, usize, InputRate)> {
        let mut buffers = Vec::new();
        let mut buffer_cache = Vec::new();

        for (index, attribute) in self.attributes.iter() {
            if buffer_cache.contains(&attribute.buffer_id) || !attribute.enabled {
                continue;
            }

            let binding = *self.buffers_binding.get(&attribute.buffer_id).unwrap();
            buffers.push((binding, attribute.stride, InputRate::Vertex));
            buffer_cache.push(attribute.buffer_id);
        }

        buffers
    }

    pub fn get_attributes_definition(&self) -> Vec<(u32, u32, AttributeInfo)> {
        let mut attributes = Vec::new();

        for (index, attribute) in self.attributes.iter() {
            if !attribute.enabled {
                continue;
            }

            let binding = *self.buffers_binding.get(&attribute.buffer_id).unwrap();
            let info = AttributeInfo {
                offset: attribute.offset,
                format: attribute.format,
            };

            attributes.push((*index, binding, info));
        }

        attributes
    }
}

pub struct GrindBufferDefinition {
    attributes: Arc<VertexAttributes>,
}

impl GrindBufferDefinition {
    pub fn new(mut attributes: Arc<VertexAttributes>) -> GrindBufferDefinition {
        Arc::make_mut(&mut attributes).generate_buffers_binding_map();
        GrindBufferDefinition { attributes }
    }
}

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
        Ok((
            self.attributes.get_buffers_definition().into_iter(),
            self.attributes.get_attributes_definition().into_iter(),
        ))
    }
}

unsafe impl VertexSource<HashMap<u32, Arc<Buffer>>> for GrindBufferDefinition {
    fn decode(
        &self,
        mut source: HashMap<u32, Arc<Buffer>>,
    ) -> (Vec<Box<BufferAccess + Send + Sync>>, usize, usize) {
        let bindings = &self.attributes.binding_buffers;
        let mut result = Vec::new();
        let mut len = 0;

        /*for i in 0..bindings.len() {
            let buffer_id = bindings.get(&(i as u32)).unwrap();
            let buffer = source.get(buffer_id).unwrap();
            let chunk =
                Box::new(buffer.chunk.as_ref().unwrap().clone()) as Box<BufferAccess + Send + Sync>;
            result.push(chunk);
            len = buffer.chunk.as_ref().unwrap().size();
        }*/

        let mut len = 36 / 4; // float
        len = len / 3; // vec3

        (result, len, 1)
    }
}

unsafe impl<T> VertexSource<Vec<T>> for GrindBufferDefinition {
    fn decode(&self, _: Vec<T>) -> (Vec<Box<BufferAccess + Sync + Send + 'static>>, usize, usize) {
        panic!("bufferless drawing should not be supplied with buffers")
    }
}

pub struct Buffer {
    //inner: CpuBufferPool<u8>,
    //pub chunk: Option<CpuBufferPoolChunk<u8, Arc<StdMemoryPool>>>,
}

impl Buffer {
    pub fn new(/*device: Arc<Device>*/) -> Buffer {
        Buffer {
            //inner: CpuBufferPool::vertex_buffer(device),
            //chunk: None,
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        //self.chunk = Some(self.inner.chunk(data.iter().cloned()).unwrap());
    }
}
