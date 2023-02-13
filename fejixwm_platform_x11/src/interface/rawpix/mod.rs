use crate::{
    *,
    errors::Result,
};


pub(crate) struct CanvasData {
    graphics_context: u32,
    
    /// Back buffer
    canvas: core::interface::rawpix::Canvas,

    /// Front buffer
    pixmap: u32,
}


pub(crate) trait WmRawpixCanvasController {

    fn new_visual_info(&self, info: &core::interface::rawpix::RawpixInfo) -> Result<X11WindowVisualInfo>;

    fn new_canvas(&mut self, wid: WindowId, info: &core::interface::rawpix::RawpixInfo) -> Result<()>;

    fn drop_canvas(&mut self, wid: WindowId);

    fn resize_canvas(&mut self, wid: WindowId, size: PixelSize) -> Result<()>;

}


impl WmRawpixCanvasController for WindowManager {

    fn drop_canvas(&mut self, wid: WindowId) {
        todo!()
    }

    fn new_canvas(&mut self, wid: WindowId, info: &core::interface::rawpix::RawpixInfo) -> Result<()> {
        todo!()
    }

    fn new_visual_info(&self, info: &core::interface::rawpix::RawpixInfo) -> Result<X11WindowVisualInfo> {
        todo!()
    }

    fn resize_canvas(&mut self, wid: WindowId, size: PixelSize) -> Result<()> {
        todo!()
    }

}