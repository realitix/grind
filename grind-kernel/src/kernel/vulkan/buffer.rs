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

// TEST NEW SOLUTION BY INDEX
pub struct VertexAttribute {
    pub enabled: bool,
    pub binding: u32,
    pub format: Format,
    pub stride: usize,
    pub offset: usize,
}

impl VertexAttribute {
    pub fn new() -> VertexAttribute {
        VertexAttribute {
            enabled: false,
            binding: 0,
            format: Format::R8Unorm,
            stride: 0,
            offset: 0,
        }
    }

    pub fn update(&mut self, binding: u32, format: Format, stride: usize, offset: usize) {
        self.binding = binding;
        self.format = format;
        self.stride = stride;
        self.offset = offset;
    }
}

pub struct VertexAttributes {
    // key = index (location)
    pub attributes: HashMap<u32, VertexAttribute>,
}

impl VertexAttributes {
    pub fn new() -> VertexAttributes {
        VertexAttributes {
            attributes: HashMap::new(),
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
        binding: u32,
        format: Format,
        stride: usize,
        offset: usize,
    ) {
        self.check_index(index);
        self.attributes
            .get_mut(&index)
            .unwrap()
            .update(binding, format, stride, offset);
    }

    pub fn enable_attribute(&mut self, index: u32) {
        self.check_index(index);
        self.attributes.get_mut(&index).unwrap().enabled = true;
    }

    pub fn disable_attribute(&mut self, index: u32) {
        self.check_index(index);
        self.attributes.get_mut(&index).unwrap().enabled = false;
    }

    // return (binding, stride, inputrate)
    pub fn get_buffers_definition(&self) -> Vec<(u32, usize, InputRate)> {
        let mut buffers = Vec::new();
        let mut binding_cache = Vec::new();

        for (index, attribute) in self.attributes.iter() {
            if binding_cache.contains(&attribute.binding) || !attribute.enabled {
                continue;
            }

            buffers.push((attribute.binding, attribute.stride, InputRate::Vertex));
            binding_cache.push(attribute.binding);
        }

        buffers
    }

    pub fn get_attributes_definition(&self) -> Vec<(u32, u32, AttributeInfo)> {
        let mut attributes = Vec::new();

        for (index, attribute) in self.attributes.iter() {
            if !attribute.enabled {
                continue;
            }

            let info = AttributeInfo {
                offset: attribute.offset,
                format: attribute.format,
            };

            attributes.push((*index, attribute.binding, info));
        }

        attributes
    }
}

// IMPL Custom Buffer Definition
pub struct AttributeDefinition {
    pub location: u32,
    pub offset: usize,
    pub format: Format,
}

pub struct AttributesDefinition {
    pub stride: usize, // vertex size
    pub attrs: Vec<AttributeDefinition>,
}

impl AttributesDefinition {
    pub fn new() -> AttributesDefinition {
        AttributesDefinition {
            stride: 0,
            attrs: Vec::new(),
        }
    }
}

pub struct GrindBufferDefinition {
    attributes: VertexAttributes,
}

impl GrindBufferDefinition {
    pub fn new() -> GrindBufferDefinition {
        GrindBufferDefinition {
            attributes: VertexAttributes::new(),
        }
    }

    pub fn set_attributes(&mut self, attributes: VertexAttributes) {
        mem::replace(&mut self.attributes, attributes);
    }

    /*pub fn set_attributes(&mut self, buffer_id: u32, location: u32, offset: usize, stride: usize, format: Format) {
        let buffer_exists = {
            match self.buffers.get(&buffer_id) {
                None => false,
                Some(x) => true
            }
        };

        if !buffer_exists {
            self.buffers.insert(buffer_id, AttributesDefinition::new());
        }
        
        let mut attributes = self.buffers.get_mut(&buffer_id).unwrap();
        attributes.stride = stride;
        attributes.attrs.push(AttributeDefinition {location, offset, format});
    }*/
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
        /*let mut buffers = Vec::new();
        let mut attributes = Vec::new();

        for (buffer_id, attrs) in self.buffers.iter() {
            let binding = *buffer_id;
            buffers.push((binding, attrs.stride, InputRate::Vertex));

            for attr in attrs.attrs.iter() {
                let info = AttributeInfo {
                    offset: attr.offset,
                    format: attr.format
                };

                attributes.push((attr.location, binding, info));
            }
        }

        Ok((buffers.into_iter(), attributes.into_iter()))*/

        /*let buffers = vec![(
            0 as u32,    // binding
            12 as usize, // vertex size (3xfloat)
            InputRate::Vertex,
        )];
        let attribs = vec![(
            0,
            0,
            AttributeInfo {
                offset: 0,
                format: Format::R32G32B32Sfloat,
            },
        )];

        Ok((buffers.into_iter(), attribs.into_iter()))*/
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
    pub definition: Arc<GrindBufferDefinition>,
}

impl Buffer {
    pub fn new(device: Arc<Device>) -> Buffer {
        Buffer {
            inner: CpuBufferPool::vertex_buffer(device),
            chunk: None,
            definition: Arc::new(GrindBufferDefinition::new()),
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.chunk = Some(self.inner.chunk(data.iter().cloned()).unwrap());
    }
}
