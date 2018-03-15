use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

pub fn is_available() -> bool {
    match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => true,
        Err(err) => false,
    }
}

pub struct VulkanDriver {
    instance: Arc<Instance>,
}

impl VulkanDriver {
    pub fn new() -> VulkanDriver {
        let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
        VulkanDriver { instance }
    }
}
