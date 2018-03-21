#![feature(ptr_internals)]

extern crate khronos;
extern crate libc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate vulkano;

pub mod egl;
mod kernel;
