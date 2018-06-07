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
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::framebuffer::RenderPass;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::framebuffer::Subpass;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::pipeline::vertex::Vertex;
use vulkano::pipeline::vertex::VertexMemberInfo;
use vulkano::pipeline::vertex::VertexMemberTy;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::acquire_next_image;
use vulkano::swapchain::present;
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::sync;
use vulkano::sync::GpuFuture;

use kernel::vulkan::shader::EmptySpecializationConstants;
use kernel::vulkan::shader::Shader;

pub struct Renderer {
    device: Arc<Device>,
    surface: Arc<Surface<Unique<c_void>>>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Unique<c_void>>>,
    swapchain_images: Vec<Arc<SwapchainImage<Unique<c_void>>>>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    last_future: Box<GpuFuture>,
    image_num: usize,
    renderpass: Arc<RenderPassAbstract + Send + Sync>,
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

        let renderpass = Arc::new(
            single_pass_renderpass!(device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            ).unwrap(),
        );

        let framebuffers = swapchain_images
            .iter()
            .map(|image| {
                Arc::new(
                    Framebuffer::start(renderpass.clone())
                        .add(image.clone())
                        .unwrap()
                        .build()
                        .unwrap(),
                ) as Arc<FramebufferAbstract + Send + Sync>
            })
            .collect::<Vec<_>>();

        let mut renderer = Renderer {
            device,
            surface,
            queue,
            swapchain,
            last_future,
            image_num,
            swapchain_images,
            framebuffers,
            renderpass,
        };

        renderer.acquire();
        renderer
    }

    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
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
            .begin_render_pass(
                self.framebuffers[self.image_num].clone(),
                false,
                vec![colors.into()],
            )
            .unwrap()
            .end_render_pass()
            .unwrap()
            .build()
            .unwrap();

        let tmp_future = Box::new(sync::now(self.device.clone()));
        let last_future = mem::replace(&mut self.last_future, tmp_future);
        last_future
            .then_execute(self.queue.clone(), cb)
            .expect("Can't execute clear command buffer")
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
        ).then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        self.acquire()
    }

    pub fn draw(&mut self, vs: &Shader, fs: &Shader) {
        struct Dummy {
            vin_position: [f32; 3],
        };
        unsafe impl Vertex for Dummy {
            fn member(name: &str) -> Option<VertexMemberInfo> {
                Some(VertexMemberInfo {
                    offset: 0,
                    ty: VertexMemberTy::F32,
                    array_size: 3,
                })
            }
        };

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Dummy>()
                .vertex_shader(vs.main_entry_point(), EmptySpecializationConstants {})
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), EmptySpecializationConstants {})
                .render_pass(Subpass::from(self.renderpass.clone(), 0).unwrap())
                .build(self.device.clone())
                .unwrap(),
        );
    }
}
