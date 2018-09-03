use libc::c_void;
use std::default::Default;
use std::ffi::{CStr, CString};
use std::ptr;
use std;

use ash::extensions::{DebugReport, Surface, Swapchain, Win32Surface, XlibSurface};
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0, V1_0};
use ash::vk;
use ash;
use ash::Entry;
use ash::Instance;

use kernel::vulkan::vulkanobject::IMAGE_ASPECT_COLOR_BIT;
use kernel::vulkan::vulkanobject::Device;
use kernel::vulkan::vulkanobject::Image;
use kernel::vulkan::vulkanobject::ImageAspectFlags;
use kernel::vulkan::vulkanobject::ImageView;
use kernel::vulkan::vulkanobject::ImageViewType;
use kernel::vulkan::vulkanobject::ImageSubresourceRange;
use kernel::vulkan::vulkanobject::Format;

#[cfg(all(unix, not(target_os = "android")))]
fn extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
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
    _: vk::uint64_t,
    _: vk::size_t,
    _: vk::int32_t,
    _: *const vk::c_char,
    p_message: *const vk::c_char,
    _: *mut vk::c_void,
) -> u32 {
    println!("{:?}", CStr::from_ptr(p_message));
    1
}


pub struct VulkanContext {
    pub entry: Entry<V1_0>,
    pub instance: Instance<V1_0>,
    pub device: Device,
    pub debug_callback: vk::DebugReportCallbackEXT,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub queue_family_index: usize,
    pub present_queue: vk::Queue,
    pub swapchain_loader: Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_image_views: Vec<ImageView>
    /*
    pub present_image_views: Vec<vk::ImageView>,

    pub pool: vk::CommandPool,
    pub draw_command_buffer: vk::CommandBuffer,
    pub setup_command_buffer: vk::CommandBuffer,

    pub depth_image: vk::Image,
    pub depth_image_view: vk::ImageView,
    pub depth_image_memory: vk::DeviceMemory,

    pub present_complete_semaphore: vk::Semaphore,
    pub rendering_complete_semaphore: vk::Semaphore*/
}

impl VulkanContext {
    fn create_instance(entry: &Entry<V1_0>, app_name: String) -> Instance<V1_0> {
        let app_name = CString::new(app_name).unwrap();
        let raw_name = app_name.as_ptr();

        let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        let extension_names_raw = extension_names();
        let appinfo = vk::ApplicationInfo {
            p_application_name: raw_name,
            s_type: vk::StructureType::ApplicationInfo,
            p_next: ptr::null(),
            application_version: 0,
            p_engine_name: raw_name,
            engine_version: 0,
            api_version: vk_make_version!(1, 0, 36),
        };
        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::InstanceCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            p_application_info: &appinfo,
            pp_enabled_layer_names: layers_names_raw.as_ptr(),
            enabled_layer_count: layers_names_raw.len() as u32,
            pp_enabled_extension_names: extension_names_raw.as_ptr(),
            enabled_extension_count: extension_names_raw.len() as u32,
        };

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        }
    }

    fn create_debug_callback<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
    ) -> vk::DebugReportCallbackEXT {
        let debug_info = vk::DebugReportCallbackCreateInfoEXT {
            s_type: vk::StructureType::DebugReportCallbackCreateInfoExt,
            p_next: ptr::null(),
            flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT
                | vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
            pfn_callback: vulkan_debug_callback,
            p_user_data: ptr::null_mut(),
        };
        let debug_report_loader =
            DebugReport::new(entry, instance).expect("Unable to load debug report");

        unsafe {
            debug_report_loader
                .create_debug_report_callback_ext(&debug_info, None)
                .expect("Unable to create debug callback")
        }
    }

    fn create_surface<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I) -> vk::SurfaceKHR {
        let win = ptr::null() as *const c_void;
        let dpy = ptr::null() as *const c_void;
        let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
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
        }
    }

    fn create_physical_device<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
        surface_loader: &Surface,
        surface: &vk::SurfaceKHR,
    ) -> (vk::PhysicalDevice, usize) {
        let pdevices = instance
            .enumerate_physical_devices()
            .expect("Physical device error");

        pdevices
            .iter()
            .map(|pdevice| {
                instance
                    .get_physical_device_queue_family_properties(*pdevice)
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let supports_graphic_and_surface = info.queue_flags
                            .subset(vk::QUEUE_GRAPHICS_BIT)
                            && surface_loader.get_physical_device_surface_support_khr(
                                *pdevice,
                                index as u32,
                                *surface,
                            );
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
        instance: &Instance<V1_0>,
        physical_device: &vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> (Device, vk::Queue) {
        let device_extension_names_raw = [Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];
        let queue_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DeviceQueueCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            queue_family_index: queue_family_index as u32,
            p_queue_priorities: priorities.as_ptr(),
            queue_count: priorities.len() as u32,
        };
        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DeviceCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: device_extension_names_raw.len() as u32,
            pp_enabled_extension_names: device_extension_names_raw.as_ptr(),
            p_enabled_features: &features,
        };

        let device: Device = unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)
                .expect("Unable to craete logical device")
        };

        let present_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        (device, present_queue)
    }

    fn create_swapchain(
        instance: &Instance<V1_0>,
        physical_device: &vk::PhysicalDevice,
        device: &Device,
        surface_loader: &Surface,
        surface: &vk::SurfaceKHR,
        swapchain_loader: &Swapchain,
        width: u32,
        height: u32
    ) -> (vk::SwapchainKHR, Vec<ImageView>) {
        let surface_formats = surface_loader
            .get_physical_device_surface_formats_khr(*physical_device, *surface)
            .unwrap();
        let surface_format = surface_formats
            .iter()
            .map(|sfmt| match sfmt.format {
                vk::Format::Undefined => vk::SurfaceFormatKHR {
                    format: vk::Format::B8g8r8Unorm,
                    color_space: sfmt.color_space,
                },
                _ => sfmt.clone(),
            })
            .nth(0)
            .expect("Unable to find suitable surface format.");
        let surface_capabilities = surface_loader
            .get_physical_device_surface_capabilities_khr(*physical_device, *surface)
            .unwrap();
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
            .subset(vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR)
        {
            vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR
        } else {
            surface_capabilities.current_transform
        };
        let present_modes = surface_loader
            .get_physical_device_surface_present_modes_khr(*physical_device, *surface)
            .unwrap();
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::Mailbox)
            .unwrap_or(vk::PresentModeKHR::Fifo);
        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SwapchainCreateInfoKhr,
            p_next: ptr::null(),
            flags: Default::default(),
            surface: *surface,
            min_image_count: desired_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: surface_resolution.clone(),
            image_usage: vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            image_sharing_mode: vk::SharingMode::Exclusive,
            pre_transform: pre_transform,
            composite_alpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            present_mode: present_mode,
            clipped: 1,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1,
            p_queue_family_indices: ptr::null(),
            queue_family_index_count: 0,
        };

        let swapchain = unsafe {
            swapchain_loader
            .create_swapchain_khr(&swapchain_create_info, None)
            .unwrap()
        };

        // Create image views
        let raw_images = swapchain_loader
            .get_swapchain_images_khr(swapchain)
            .unwrap();
        let mut image_views: Vec<ImageView> = Vec::new();
        for raw_image in raw_images.into_iter() {
            let image = Image::from_swapchain_image(
                raw_image, surface_resolution.width,
                surface_resolution.height, surface_format.format);
            let subresource_range = ImageSubresourceRange {
                aspect_mask: IMAGE_ASPECT_COLOR_BIT,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            };
            let image_view = ImageView::from_device(
                device, image, ImageViewType::Type2d,
                surface_format.format, subresource_range);
            image_views.push(image_view);
        }

        (swapchain, image_views)
    }

    pub fn new(app_name: String) -> VulkanContext {
        let entry = Entry::new().unwrap();
        let instance = VulkanContext::create_instance(&entry, app_name);
        let debug_callback = VulkanContext::create_debug_callback(&entry, &instance);
        let surface = VulkanContext::create_surface(&entry, &instance);
        let surface_loader = Surface::new(&entry, &instance).expect("Unable to load the Surface extension");
        let (physical_device, queue_family_index) =
            VulkanContext::create_physical_device(&entry, &instance, &surface_loader, &surface);
        let (device, present_queue) =
            VulkanContext::create_device(&instance, &physical_device, queue_family_index as u32);
        let swapchain_loader = Swapchain::new(&instance, &device).expect("Unable to load swapchain");
        let (swapchain, swapchain_image_views) = VulkanContext::create_swapchain(
            &instance, &physical_device, &device, &surface_loader, &surface,
            &swapchain_loader, 200, 200);
        
        VulkanContext {
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
            swapchain_image_views
        }
    }
}