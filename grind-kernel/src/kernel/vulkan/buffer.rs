use std::sync::Arc;

use vulkano::buffer::cpu_pool::CpuBufferPoolChunk;
use vulkano::buffer::CpuBufferPool;
use vulkano::device::Device;
use vulkano::memory::pool::StdMemoryPool;

pub struct Buffer {
    inner: CpuBufferPool<u8>,
    chunk: Option<CpuBufferPoolChunk<u8, Arc<StdMemoryPool>>>,
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
