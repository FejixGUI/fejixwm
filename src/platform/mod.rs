#[cfg(feature = "platform-x11")]
#[path = "x11/mod.rs"]
pub mod imlplementation;

#[cfg(not(feature = "impl"))]
#[path = "null/mod.rs"]
pub mod implementation;

pub use self::implementation::*;