use crate::{
    *,
    errors::*
};

pub use crate::core::interface::window_manip::*;


impl VisibilityController for X11ShellClient {

    fn set_visible(&self, window: &mut Self::Window, visible: bool) -> Result<()> {
        if visible {
            self.connection.send_and_check_request(&xcb::x::MapWindow {
                window: window.handle
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot map window")))?;
        }
        else {
            self.connection.send_and_check_request(&xcb::x::UnmapWindow {
                window: window.handle
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot unmap window")))?;
        }
        
        Ok(())
    }

}


impl TitleController for X11ShellClient {

    fn set_title(&self, window: &mut Self::Window, title: &str) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: window.handle,
            property: self.atoms._NET_WM_NAME,
            r#type: self.atoms.UTF8_STRING,
            data: title.as_bytes()
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set title")))?;
        
        Ok(())
    }

}