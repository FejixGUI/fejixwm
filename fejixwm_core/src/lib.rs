/*!
## Purpose

This crate contains platform-independent code of FejixWM.

## Definitions

### General

* **Shell** - an operating system's graphical environment.
  Typical examples of shells are Microsoft Windows Shell, GNOME Shell, KDE Plasma.

* **Surface** - a rectangular pixel array displayed on screen by a shell.
  Surfaces are always managed by shells and are rarely directly accessible to programs.

* **Window** - an object provided by a shell to a program giving indirect access to a surface, its decorations
  and additional functionality like receiving user input (keyboard presses, mouse movement etc.).

* **Platform** - a shell API, e.g. Windows API, Cocoa, X11, Wayland.

* **Graphics API** - an API for displaying graphics on a window's surface, e.g. OpenGL or Vulkan.
  FejixWM provides a software rendering API called Rawpix.

### Specific to FejixWM

* **Canvas** - an object used to display graphics, e.g. OpenGL context or Vulkan surface.
  Every window must have at most one canvas.

* **WM subsystem** - specific WM functionality that can be turned ON and OFF.

*/

#![allow(dead_code)]
#![allow(unused_imports)]

mod core;
pub mod errors;
pub mod events;
pub mod interface;

pub use self::core::*;
