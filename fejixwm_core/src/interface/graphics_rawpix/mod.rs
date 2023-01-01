#![allow(non_camel_case_types)]

use crate::*;


/// All components are specified in little-endian order.
/// Not all formats may be supported (typically, only a few are implemented by a platform).
/// Size of a pixel of a specific format can be determined by [size_of_pixel].
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


pub fn size_of_pixel(format: PixelFormat) -> usize {
    match format {
        PixelFormat::RGB_888 | PixelFormat::BGR_888 => 3,
        _ => 4
    }
}


pub struct SurfaceParams {
    format: PixelFormat
}


pub struct Pixmap {
    format: PixelFormat,

    /// Size of row in bytes (including padding bytes).
    /// Equal to `width * size_of_pixel(format) + padding`
    pub stride: usize,

    /// Number of padding bytes after each row.
    /// Equal to `stride - width * size_of_pixel(format)`
    pub padding: usize,

    /// Pixel data. Contains `height * (width * size_of_pixel(format) + padding)` bytes.
    /// The length of this vector MUST NOT be modified
    pub pixels: Vec<u8>
}


pub trait Surface : SurfaceTrait {

    fn get_pixmap<'a>(&'a mut self) -> &'a mut Pixmap;

}