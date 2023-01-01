use crate::*;


pub enum OpenglProfile {
    Core,
    Compatibility,
}


pub struct OpenglVersion {
    pub major_version: u8,
    pub minor_version: u8,
    pub profile: OpenglProfile,
}


impl OpenglVersion {
    pub fn new(major_version: u8, minor_version: u8, profile: OpenglProfile) -> Self {
        Self { major_version, minor_version, profile }
    }
}


pub struct SurfaceParams {
    pub version: OpenglVersion,

    pub red_bits: u8,
    pub green_bits: u8,
    pub blue_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,

    pub multisamples: u8,

    pub double_buffer: bool,
}


pub trait OpenglContext : SurfaceTrait {



}