pub extern crate fejixwm_core;

pub use fejixwm_core::{
    Error, Result,
    
    core::{
        *, traits::*
    },
    
    events,
    
    self as core
};