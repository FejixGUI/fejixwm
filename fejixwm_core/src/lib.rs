/*!
## Purpose

This crate defines global platform-independent type traits and their relationships.
All platform implementation crates have to implement the traits.


## Definitions

**Shell** is an operating system's graphical environment. Typical examples of shells are Microsoft Windows Shell, GNOME Shell, KDE Plasma.

**Platform** is a shell API.

The following definitions are parts of every platform, thus they are platform-specific:

**App** is a global singleton that represents a shell client.

**Window** is a rectangular graphical surface displayed by the shell. Windows interoperate with the shell in various 
different ways. For example, windows can accept user input like keyboard presses and mouse button clicks.

**Surface** is an object which a program can use to display graphics within a window.

**Graphics API** is a concrete protocol for displaying graphics on a surface. Even though there are cross-platform
graphics APIs (indicating that they are supposed to be platform-independent), there are numerous platform-specific 
(platform-dependent) nuances about the APIs. Therefore, every platform should provide a separate implementation of such
APIs.
*/

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod core;
pub mod errors;
pub mod events;
pub mod interface;

pub use self::{
    core::{*, traits::*},
    errors::{Result, Error}
};
