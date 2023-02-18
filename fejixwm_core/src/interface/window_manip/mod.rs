use crate::{
    errors::Result,
    *,
};


pub trait TitleController : ShellClientTrait {

    fn set_title(&self, window: &mut Self::Window, title: &str) -> Result<()>;

}


pub trait VisibilityController : ShellClientTrait {

    fn set_visible(&self, window: &mut Self::Window, visible: bool) -> Result<()>;

}