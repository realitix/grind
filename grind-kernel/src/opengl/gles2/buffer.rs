use kernel::vulkan::buffer::Buffer as VulkanBuffer;
use kernel::vulkan::VulkanDriver;
use opengl::types::*;
use std::slice;
use std::sync::Arc;

pub struct Buffer {
    pub id: GLuint,
    pub target: GLenum,
    pub inner: Arc<VulkanBuffer>,
    vertex_attrib_array: Vec<GLuint>,
}

impl Buffer {
    pub fn new(id: GLuint, kernel: &VulkanDriver) -> Buffer {
        Buffer {
            id,
            target: 0,
            inner: Arc::new(kernel.new_buffer()),
            vertex_attrib_array: Vec::new(),
        }
    }

    pub fn buffer_data(
        &mut self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const GLvoid,
        usage: GLenum,
    ) {
        let buf = unsafe { slice::from_raw_parts::<u8>(data as *const u8, size as usize) };
        Arc::get_mut(&mut self.inner).unwrap().set_data(buf);
    }

    pub fn enable_vertex_attrib_array(&mut self, index: GLuint) {
        self.vertex_attrib_array.push(index);
    }

    pub fn get_buffer(&self) -> Arc<VulkanBuffer> {
        self.inner.clone()
    }
}
