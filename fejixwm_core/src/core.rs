use crate::{
    errors::Result,
    events::EventHandler,
};

use bitflags::bitflags;


#[derive(Clone)]
pub struct PixelSize {
    pub width: u32,
    pub height: u32
}

impl PixelSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl std::fmt::Display for PixelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.width, self.height)
    }
}


bitflags! {
    pub struct WindowFlags : u32 {
        const RESIZABLE         = 0b_0000_0000_0000_0000_0000_0000_0000_0001;
        const SMOOTH_REDRAW     = 0b_0000_0000_0000_0000_0000_0000_0000_0010;
        const TEXT_INPUT        = 0b_0000_0000_0000_0000_0000_0000_0000_0100;
    }
}


pub type WindowId = u32;


#[derive(Clone)]
pub struct WindowInfo<'a> {
    pub id: WindowId,
    pub flags: WindowFlags,
    pub size: PixelSize,

    pub canvas_info: CanvasInfo<'a>,
}


#[derive(Clone)]
pub enum CanvasInfo<'a> {
    None,
    RawpixInfo(&'a crate::interface::rawpix::CanvasInfo)
}


pub struct WindowManagerInfo<'a> {
    pub name: &'a str,
}


pub trait WindowManagerTrait : Sized {
    
    fn new(info: &WindowManagerInfo) -> Result<Self>;

    fn new_window(&mut self, info: &WindowInfo) -> Result<()>;

    fn drop_window(&mut self, wid: WindowId) -> Result<()>;

    fn run<F: EventHandler<Self>>(&mut self, event_handler: F);

    fn stop(&mut self);

}