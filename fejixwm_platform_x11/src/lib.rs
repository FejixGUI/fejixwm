#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate fejixwm_core;
pub(crate) use fejixwm_core as core;

extern crate x11;
extern crate xcb;

mod types;
mod shell_client_impl;
mod subsystems_impl;
pub mod implementation;


pub use types::*;