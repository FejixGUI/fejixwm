#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate fejixwm_core;
pub(crate) use fejixwm_core as core;

extern crate x11;
extern crate xcb;

mod types;
mod wm_impl;
mod wm_impl_events;
mod wm_impl_traits;
mod interface;


pub use types::*;