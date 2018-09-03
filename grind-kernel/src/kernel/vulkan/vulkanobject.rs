use std::ptr;

use ash;
use ash::vk;
use ash::version::{InstanceV1_0, DeviceV1_0, V1_0};

use kernel::vulkan::vulkancontext::VulkanContext;

// ----------
// FLAGS
// ----------
pub use ash::vk::types::IMAGE_ASPECT_COLOR_BIT as IMAGE_ASPECT_COLOR_BIT;

// ----------
// SIMPLE TYPES
// ----------
pub use ash::vk::types::ComponentMapping as ComponentMapping;
pub use ash::vk::types::ComponentSwizzle as ComponentSwizzle;
pub use ash::vk::types::Extent3D as Extent3D;
pub use ash::vk::types::Format as Format;
pub use ash::vk::types::ImageAspectFlags as ImageAspectFlags;
pub use ash::vk::types::ImageCreateFlags as ImageCreateFlags;
pub use ash::vk::types::ImageCreateInfo as ImageCreateInfo;
pub use ash::vk::types::ImageLayout as ImageLayout;
pub use ash::vk::types::ImageSubresourceRange as ImageSubresourceRange;
pub use ash::vk::types::ImageTiling as ImageTiling;
pub use ash::vk::types::ImageType as ImageType;
pub use ash::vk::types::ImageUsageFlags as ImageUsageFlags;
pub use ash::vk::types::ImageViewCreateFlags as ImageViewCreateFlags;
pub use ash::vk::types::ImageViewCreateInfo as ImageViewCreateInfo;
pub use ash::vk::types::ImageViewType as ImageViewType;
pub use ash::vk::types::MemoryAllocateInfo as MemoryAllocateInfo;
pub use ash::vk::types::MemoryPropertyFlags as MemoryPropertyFlags;
pub use ash::vk::types::SampleCountFlags as SampleCountFlags;
pub use ash::vk::types::SharingMode as SharingMode;
pub use ash::vk::types::StructureType as StructureType;

// -----------
// LOW LEVEL TYPES
// -----------
pub type Device = ash::Device<V1_0>;


// ----------
// FUNCTIONS
// ----------
fn find_memory_type(context: &VulkanContext, type_filter: u32, properties: MemoryPropertyFlags) -> u32 {
    let cache_properties = context.instance.get_physical_device_memory_properties(context.physical_device);
    for (i, memory_type) in cache_properties.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) != 0 && (memory_type.property_flags & properties) == properties {
            return i as u32;
        }
    }

    0
}

// ----------
// HIGH-LEVEL STRUCTS
// ----------
pub struct Image {
    image : vk::types::Image,
    memory: vk::types::DeviceMemory,
    image_format: Format,
    width: u32,
    height: u32,
    depth: u32,
    mip_levels: u32
}

impl Image {
    pub fn new(context: &VulkanContext, image_type: ImageType, image_format: Format,
               width: u32, height: u32, depth: u32, mip_levels: u32, layers: u32,
               samples: SampleCountFlags, sharing_mode: SharingMode, tiling: ImageTiling,
               usage: ImageUsageFlags, layout: ImageLayout, queue_families: Vec<u32>,
               memory_properties: MemoryPropertyFlags) -> Image{

        // Check image can be created
        context.instance.get_physical_device_image_format_properties(
            context.physical_device, image_format, image_type, tiling,
            usage, ImageCreateFlags::empty())
            .expect("Can't create your image");

        // Create image
        let extent = Extent3D { width, height, depth };
        let image_create = ImageCreateInfo {
            s_type: StructureType::ImageCreateInfo,
            p_next: ptr::null(),
            flags: ImageCreateFlags::empty(),
            image_type: image_type,
            format: image_format,
            extent: extent,
            mip_levels: mip_levels,
            array_layers: layers,
            samples: samples,
            tiling: tiling,
            usage: usage,
            sharing_mode: sharing_mode,
            queue_family_index_count: queue_families.len() as u32,
            p_queue_family_indices: queue_families.as_ptr() as *const u32,
            initial_layout: layout
        };

        let image = unsafe { context.device.create_image(&image_create, None).unwrap() };

        // Create memory
        let requirements = context.device.get_image_memory_requirements(image);
        let alloc_info = MemoryAllocateInfo {
            s_type: StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: requirements.size,
            memory_type_index: find_memory_type(context, requirements.memory_type_bits, memory_properties)
        };
        let memory = unsafe { context.device.allocate_memory(&alloc_info, None).unwrap() };

        // Bind memory to image
        unsafe { context.device.bind_image_memory(image, memory, 0).unwrap() };

        Image { image, memory: memory, image_format, width, height, depth, mip_levels }
    }

    pub fn from_swapchain_image(swapchain_image: vk::types::Image, width: u32,
                                height: u32, image_format: Format) -> Image {
        Image {
            image: swapchain_image,
            memory: vk::types::DeviceMemory::null(),
            image_format: image_format,
            width: width,
            height: height,
            depth: 1,
            mip_levels: 1
        }           
    }
}


pub struct ImageView {
    image: Image,
    image_view: vk::types::ImageView
}

impl ImageView {
    pub fn new(context: &VulkanContext, image: Image, view_type: ImageViewType,
               format: Format, subresource_range: ImageSubresourceRange) -> ImageView {
        ImageView::from_device(&context.device, image, view_type, format, subresource_range)
    }

    pub fn from_device(device: &Device, image: Image, view_type: ImageViewType,
                       format: Format, subresource_range: ImageSubresourceRange) -> ImageView {
        let components = ComponentMapping {
            r: ComponentSwizzle::Identity,
            g: ComponentSwizzle::Identity,
            b: ComponentSwizzle::Identity,
            a: ComponentSwizzle::Identity
        };

        let image_view_create = ImageViewCreateInfo {
            s_type: StructureType::ImageViewCreateInfo,
            p_next: ptr::null(),
            flags: ImageViewCreateFlags::empty(),
            image: image.image,
            view_type: view_type,
            format: format,
            components: components,
            subresource_range: subresource_range
        };

        let image_view = unsafe { device.create_image_view(&image_view_create, None).unwrap() };

        ImageView { image, image_view }
    }
}