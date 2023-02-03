#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate fejixwm_core;
pub(crate) use fejixwm_core as core;

extern crate x11;
extern crate xcb;

pub(crate) mod types;
pub(crate) mod wm_impl;
pub(crate) mod wm_impl_events;
pub(crate) mod wm_impl_traits;


pub use types::*;