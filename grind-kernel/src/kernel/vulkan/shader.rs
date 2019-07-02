use std::sync::Arc;

use kernel::vulkan::vulkancontext::VulkanContext;
use kernel::vulkan::vulkanobject::ShaderModule;
use kernel::vulkan::vulkanobject::ShaderStageFlags;


// **********
// Shader
// **********
pub struct Shader {
    pub module: Arc<ShaderModule>,
    pub shader_stage: ShaderStageFlags,
}

impl Shader {
    pub fn new(context: &VulkanContext, spirv: &[u32], shader_stage: ShaderStageFlags) -> Shader {
        let module = ShaderModule::new(context, spirv);
        Shader {
            module: Arc::new(module),
            shader_stage: shader_stage,
        }
    }
}
