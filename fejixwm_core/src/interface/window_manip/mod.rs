use crate::{
    errors::Result,
    *,
};


pub trait WmTitleController {

    fn set_title(&mut self, wid: WindowId, title: &str) -> Result<()>;

}


pub trait WmVisibilityController {

    fn set_visible(&mut self, wid: WindowId, visible: bool) -> Result<()>;

}