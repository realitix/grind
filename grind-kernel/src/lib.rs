#![feature(ptr_internals)]
#![feature(vec_remove_item)]
#![feature(thread_local)]

extern crate khronos;
extern crate libc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate vulkano;

pub mod egl;
mod kernel;
