#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate fejixwm_core;
pub(crate) use fejixwm_core as core;

extern crate x11;
extern crate xcb;

pub mod types;
pub mod types_impl;
pub mod traits_impl;


pub use types::*;