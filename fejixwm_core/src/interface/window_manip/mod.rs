use crate::*;


pub trait WmTitleController {

    fn set_title(&self, wid: WindowId, title: &str) -> Result<()>;

}


pub trait WmVisibilityController {

    fn set_visible(&self, wid: WindowId, visible: bool) -> Result<()>;

}