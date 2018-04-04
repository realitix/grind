pub mod vulkan;


pub trait Kernel {
    fn is_available(&self) -> bool;
}
