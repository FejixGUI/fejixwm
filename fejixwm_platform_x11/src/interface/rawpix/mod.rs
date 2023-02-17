use crate::{
    *,
    errors::Result,
};


pub(crate) struct CanvasData {
    graphics_context: u32,
    
    /// Back buffer
    rawpix_data: core::interface::rawpix::RawpixData,

    /// Front buffer
    pixmap: u32,
}

