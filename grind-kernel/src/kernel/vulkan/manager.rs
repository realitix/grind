use kernel::vulkan::vulkancontext::Vulkancontext;

// Allow to retrieve the current image to draw
pub struct ImageManager {

}

impl ImageManager {
    pub fn aquire_next_image(context: &VulkanContext) {
        let index = context.aquire_next_image();
    }
}