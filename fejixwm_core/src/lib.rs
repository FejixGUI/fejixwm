#![allow(dead_code)]
#![allow(unused_imports)]

pub mod core;
pub mod errors;
pub mod events;
pub mod interface;

/// Writing `use fejixwm::prelude::*;` will include the most common things like `App` and `Window`.
pub mod prelude {
    pub use crate::core::*;
}

pub use self::prelude::*;
pub use self::errors::{Result, Error};
