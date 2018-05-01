use libc::c_void;
use std::boxed::Box;
use std::mem;
use std::ptr::Unique;
use std::sync::Arc;

use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::format::ClearValue;
use vulkano::framebuffer::RenderPass;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::swapchain::acquire_next_image;
use vulkano::swapchain::present;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::sync;
use vulkano::sync::GpuFuture;

pub struct Renderer {
    device: Arc<Device>,
    surface: Arc<Surface<Unique<c_void>>>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Unique<c_void>>>,
    swapchain_images: Vec<Arc<SwapchainImage<Unique<c_void>>>>,
    last_future: Box<GpuFuture>,
    image_num: usize,
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        surface: Arc<Surface<Unique<c_void>>>,
        queue: Arc<Queue>,
        swapchain: Arc<Swapchain<Unique<c_void>>>,
        swapchain_images: Vec<Arc<SwapchainImage<Unique<c_void>>>>,
    ) -> Renderer {
        let last_future = Box::new(sync::now(device.clone()));
        let image_num = 0;

        let mut renderer = Renderer {
            device,
            surface,
            queue,
            swapchain,
            last_future,
            image_num,
            swapchain_images,
        };

        renderer.acquire();
        renderer
    }

    fn acquire(&mut self) {
        let (image_num, acquire_future) = match acquire_next_image(self.swapchain.clone(), None) {
            Ok(r) => r,
            Err(err) => panic!("Grind acquire error: {:?}", err),
        };
        //acquire_future.then_signal_fence_and_flush().unwrap().wait(None).unwrap();
        self.last_future = Box::new(acquire_future);
        self.image_num = image_num;
    }

    pub fn clear(&mut self, colors: [f32; 4]) {
        let clear_value = ClearValue::Float(colors);
        let cb = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        ).unwrap()
            .clear_color_image(self.swapchain_images[self.image_num].clone(), clear_value)
            .expect("Clear color error")
            .build()
            .unwrap();

        // let future = cb.execute(self.queue.clone()).expect("Can't execute clear command buffer");
        // future.then_signal_fence_and_flush().unwrap().wait(None).unwrap();
        let tmp_future = Box::new(sync::now(self.device.clone()));
        let last_future = mem::replace(&mut self.last_future, tmp_future);
        last_future
            .then_execute(self.queue.clone(), cb)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();
    }

    pub fn present(&mut self) {
        present(
            self.swapchain.clone(),
            sync::now(self.device.clone()),
            self.queue.clone(),
            self.image_num,
        );
        self.acquire()
    }
}
