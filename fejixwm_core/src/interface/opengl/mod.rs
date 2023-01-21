use crate::*;


pub struct CanvasInfo {
    pub major_version: u8,
    pub minor_version: u8,
    pub compatibility_flag: bool,

    pub red_bits: u8,
    pub green_bits: u8,
    pub blue_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,

    pub multisamples: u8,

    /// Set to 2 for double-buferring
    pub framebuffers: u8,
}


pub struct Canvas {

}