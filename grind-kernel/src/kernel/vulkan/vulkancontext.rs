use std::os::raw::{c_void, c_char};
use std::default::Default;
use std::ffi::{CStr, CString};
use std::ptr;
use std;

use ash::extensions::khr::{Surface, Swapchain, Win32Surface, WaylandSurface, XlibSurface};
use ash::extensions::ext::DebugReport;
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
use ash::{Entry, Instance, Device};

use kernel::vulkan::vulkanobject::ImageViewType;
use kernel::vulkan::vulkanobject::ImageSubresourceRange;
use kernel::vulkan::vulkanobject::Fence;

use kernel::vulkan::vulkanobject as vo;

#[cfg(all(unix, not(target_os = "android")))]
fn extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        WaylandSurface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}

#[cfg(all(windows))]
fn extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    println!("{:?}", CStr::from_ptr(p_message));
    1
}


pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub debug_callback: vk::DebugReportCallbackEXT,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub queue_family_index: usize,
    pub present_queue: vk::Queue,
    pub swapchain_loader: Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_image_views: Vec<vo::ImageView>,
    pub current_swapchain_image: u32
}

impl VulkanContext {
    fn create_instance(entry: &Entry, app_name: String) -> Instance {
        let app_name = CString::new(app_name).unwrap();
        let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        let extension_names_raw = extension_names();
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(vk_make_version!(1, 0, 36));

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names_raw);

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        }
    }

    fn create_debug_callback(entry: &Entry, instance: &Instance) -> vk::DebugReportCallbackEXT {
        let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
            .flags(vk::DebugReportFlagsEXT::ERROR | vk::DebugReportFlagsEXT::WARNING | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING)
            .pfn_callback(Some(vulkan_debug_callback));
        
        let debug_report_loader = DebugReport::new(entry, instance);

        unsafe {
            debug_report_loader
                .create_debug_report_callback(&debug_info, None)
                .expect("Unable to create debug callback")
        }
    }

    fn create_surface(entry: &Entry, instance: &Instance, display: *const c_void, surface: *const c_void) -> vk::SurfaceKHR {
        let wayland_create_info = vk::WaylandSurfaceCreateInfoKHR::builder()
            .display(display as *mut _)
            .surface(surface as *mut _);
 
        let wayland_surface_loader = WaylandSurface::new(entry, instance);

        unsafe {
            wayland_surface_loader
                .create_wayland_surface(&wayland_create_info, None)
                .expect("Unable to create wayland surface")
        }


        /*let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
            s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            window: win as vk::Window,
            dpy: dpy as *mut vk::Display,
        };
        let xlib_surface_loader =
            XlibSurface::new(entry, instance).expect("Unable to load xlib surface");

        unsafe {
            xlib_surface_loader
                .create_xlib_surface_khr(&x11_create_info, None)
                .expect("Unable to create Surface")
        }*/
    }

    fn create_physical_device<I: InstanceV1_0>(
        instance: &I,
        surface_loader: &Surface,
        surface: &vk::SurfaceKHR,
    ) -> (vk::PhysicalDevice, usize) {
        let pdevices = unsafe {
            instance
            .enumerate_physical_devices()
            .expect("Physical device error")
        };
        
        pdevices
            .iter()
            .map(|pdevice| {
                let properties = unsafe { instance.get_physical_device_queue_family_properties(*pdevice) };
                properties
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let surface_supported = unsafe {
                            surface_loader.get_physical_device_surface_support(
                                *pdevice,
                                index as u32,
                                *surface,
                            )
                        };
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                            && surface_supported;
                        
                        match supports_graphic_and_surface {
                            true => Some((*pdevice, index)),
                            _ => None,
                        }
                    })
                    .nth(0)
            })
            .filter_map(|v| v)
            .nth(0)
            .expect("Couldn't find suitable device.")
    }

    fn create_device(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> (Device, vk::Queue) {
        let device_extension_names_raw = [Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];
        let queue_info = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&priorities)
            .build()];

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device: Device = unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)
                .expect("Unable to create logical device")
        };

        let present_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        (device, present_queue)
    }

    fn create_swapchain(
        physical_device: &vk::PhysicalDevice,
        device: &Device,
        surface_loader: &Surface,
        surface: &vk::SurfaceKHR,
        swapchain_loader: &Swapchain,
        width: u32,
        height: u32
    ) -> (vk::SwapchainKHR, Vec<vo::ImageView>) {
        let surface_formats = unsafe {
            surface_loader
            .get_physical_device_surface_formats(*physical_device, *surface)
            .unwrap()
        };
        let surface_format = surface_formats
            .iter()
            .map(|sfmt| match sfmt.format {
                vo::Format::UNDEFINED => vk::SurfaceFormatKHR {
                    format: vk::Format::B8G8R8_UNORM,
                    color_space: sfmt.color_space,
                },
                _ => sfmt.clone(),
            })
            .nth(0)
            .expect("Unable to find suitable surface format.");
            
        let surface_capabilities = unsafe {
            surface_loader
            .get_physical_device_surface_capabilities(*physical_device, *surface)
            .unwrap()
        };

        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }
        let surface_resolution = match surface_capabilities.current_extent.width {
            std::u32::MAX => vk::Extent2D {width, height},
            _ => surface_capabilities.current_extent,
        };
        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = unsafe {
            surface_loader
            .get_physical_device_surface_present_modes(*physical_device, *surface)
            .unwrap()
        };

        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let image_usage = vo::ImageUsageFlags::COLOR_ATTACHMENT | vo::ImageUsageFlags::TRANSFER_DST | vo::ImageUsageFlags::TRANSFER_SRC;
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution.clone())
            .image_usage(image_usage)
            .image_sharing_mode(vo::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain = unsafe {
            swapchain_loader
            .create_swapchain(&swapchain_create_info, None)
            .unwrap()
        };

        let raw_images = unsafe {
            swapchain_loader
            .get_swapchain_images(swapchain)
            .unwrap()
        };

        // Create image views
        let mut image_views: Vec<vo::ImageView> = Vec::new();
        for raw_image in raw_images.into_iter() {
            let image = vo::Image::from_swapchain_image(
                raw_image, surface_resolution.width,
                surface_resolution.height, surface_format.format);
            let subresource_range = ImageSubresourceRange {
                aspect_mask: vo::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            };
            let image_view = vo::ImageView::from_device(
                device, image, ImageViewType::TYPE_2D,
                surface_format.format, subresource_range);
            image_views.push(image_view);
        }

        (swapchain, image_views)
    }

    fn update_swapchain_layout(&self) {
        for image_view in self.swapchain_image_views.iter() {
            let swapchain_image = &image_view.image;
            vo::immediate_buffer(self, |cmd| {
                 cmd.update_image_layout(
                    self, swapchain_image, vo::ImageLayout::UNDEFINED,
                    vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vo::PipelineStageFlags::ALL_GRAPHICS,
                    vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vo::AccessFlags::empty(),
                      vo::AccessFlags::COLOR_ATTACHMENT_WRITE, 0, 1);
            });
        }
    }

    pub fn new(display_ptr: *const c_void, surface_ptr: *const c_void, width: u32, height: u32) -> VulkanContext {
        let app_name = "TEST".to_string();
        let entry = Entry::new().unwrap();
        let instance = VulkanContext::create_instance(&entry, app_name);
        let debug_callback = VulkanContext::create_debug_callback(&entry, &instance);
        let surface = VulkanContext::create_surface(&entry, &instance, display_ptr, surface_ptr);
        let surface_loader = Surface::new(&entry, &instance);
        let (physical_device, queue_family_index) =
            VulkanContext::create_physical_device(&instance, &surface_loader, &surface);
        let (device, present_queue) =
            VulkanContext::create_device(&instance, &physical_device, queue_family_index as u32);
        let swapchain_loader = Swapchain::new(&instance, &device);
        
        let (swapchain, swapchain_image_views) = VulkanContext::create_swapchain(
            &physical_device, &device, &surface_loader, &surface,
            &swapchain_loader, width, height);
        
        let mut context = VulkanContext {
            entry,
            instance,
            device,
            debug_callback,
            surface,
            physical_device,
            queue_family_index,
            present_queue,
            swapchain_loader,
            swapchain,
            swapchain_image_views,
            current_swapchain_image: 0
        };

        let signal_semaphore = context.acquire();
        context.update_swapchain_layout();
        context
    }

    pub fn acquire(&mut self) -> vo::Semaphore {
        let sem = vo::Semaphore::new(self);
        self.current_swapchain_image  = unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain, u64::max_value(), sem.semaphore, Fence::null()).unwrap().0
        };

        sem
    }

    pub fn get_current_image(&self) -> vo::Image {
        self.swapchain_image_views[self.current_swapchain_image as usize].image.clone()
    }

    pub fn wait_device_idle(&self) {
        unsafe { self.device.device_wait_idle().unwrap() };
    }

    pub fn present(&mut self, semaphores: &[vk::Semaphore]) {
        let swapchain_image = &self.swapchain_image_views[self.current_swapchain_image as usize].image;

        // Pass to present layout
        vo::immediate_buffer(self, |cmd| {
            cmd.update_image_layout(
                self, swapchain_image, vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vo::ImageLayout::PRESENT_SRC_KHR, vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vo::PipelineStageFlags::ALL_GRAPHICS, vo::AccessFlags::COLOR_ATTACHMENT_WRITE,
                vo::AccessFlags::MEMORY_READ, 0, 1);
        });

        let swapchains = [self.swapchain];
        let indices = [self.current_swapchain_image];
        
        let create_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(semaphores)
            .swapchains(&swapchains)
            .image_indices(&indices);

        unsafe { self.swapchain_loader.queue_present(self.present_queue, &create_info).unwrap() };
        self.wait_device_idle();

        // Switch back to color buffer
        vo::immediate_buffer(self, |cmd| {
            cmd.update_image_layout(
                self, swapchain_image, 
                vo::ImageLayout::PRESENT_SRC_KHR, vo::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vo::PipelineStageFlags::ALL_GRAPHICS, vo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vo::AccessFlags::MEMORY_READ, vo::AccessFlags::COLOR_ATTACHMENT_WRITE, 
                0, 1);
        });
    }
}
