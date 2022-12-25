#![allow(dead_code)]
#![allow(unused_imports)]

pub mod interface;

mod platform;

/// Writing `use fejixwm::prelude::*;` will include the most common things like `App` and `Window`.
pub mod prelude {
    pub use crate::interface::core::*;
}

pub use self::prelude::*;
pub use self::interface::errors::{Result, Error};
