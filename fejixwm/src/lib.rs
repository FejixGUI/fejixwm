#![allow(unused_imports)]

pub extern crate fejixwm_core;

pub use fejixwm_core::{
    self as core,
    *,
    events,
    interface,
};

#[cfg(not(feature = "_platform_impl"))]
compile_error!("No platform was selected");

#[cfg(feature = "platform_x11")]
pub extern crate fejixwm_platform_x11;
#[cfg(feature = "platform_x11")]
pub use fejixwm_platform_x11 as platform;

#[cfg(feature = "_platform_impl")]
pub use platform::*;