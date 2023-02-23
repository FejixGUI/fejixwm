#![allow(non_camel_case_types)]

use crate::{
    errors::Result,
    *
};

use std::{
    rc::Rc,
    cell::RefCell,
};


/// All components are specified in little-endian order.
/// Not all formats may be supported (typically, only a few are implemented by a platform).
#[derive(Clone)]
pub enum PixelFormat {
    /// Red 8 bits, Green 8 bits, Blue 8 bits (packed)
    RGB_888,
    /// Blue 8 bits, Green 8 bits, Red 8 bits (packed)
    BGR_888,
    /// Red 8 bits, Green 8 bits, Blue 8 bits, unused 8 bits
    RGBX_8888,
    /// unused 8 bits, Red 8 bits, Green 8 bits, Blue 8 bits
    XRGB_8888,
    /// Blue 8 bits, Green 8 bits, Red 8 bits, unused 8 bits
    BGRX_8888,
    /// unused 8 bits, Blue 8 bits, Green 8 bits, Red 8 bits
    XBGR_8888,
    /// Red 8 bits, Green 8 bits, Blue 8 bits, Alpha 8 bits
    RGBA_8888,
    /// Alpha 8 bits, Red 8 bits, Green 8 bits, Blue 8 bits
    ARGB_8888,
    /// Blue 8 bits, Green 8 bits, Red 8 bits, Alpha 8 bits
    BGRA_8888,
    /// Alpha 8 bits, Blue 8 bits, Green 8 bits, Red 8 bits
    ABGR_8888,
}


#[derive(Clone)]
pub struct RawpixInfo {
    
    format: PixelFormat

}


/// Access to the pixel data is thread-unsafe
#[derive(Clone)]
pub struct RawpixData {

    pub format: PixelFormat,

    /// Number of padding bytes added after each row.
    pub padding: usize,

    /// Pixel data. Contains `height * (width * format.size_of_pixel() + padding)` bytes.
    pub pixels: Rc<RefCell<Box<[u8]>>>,

}


pub trait RawpixCanvasTrait : CanvasTrait {

    fn get_raw_pixel_data(&self) -> RawpixData;

    /// Copies the back buffer to the front buffer
    fn present(
        &self,
        client: &Self::ShellClient,
        window: &mut Self::Window
    ) -> Result<()>;

}


impl PixelFormat {
    pub fn size_of_pixel(&self) -> usize {
        match self {
            Self::RGB_888 | Self::BGR_888 => 3,
            _ => 4
        }
    }
}