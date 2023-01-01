use crate::*;


pub trait WindowWithTitle : WindowTrait {

    fn set_title(&self, title: &str) -> Result<()>;

}