use crate::types::*;


impl X11GlobalTextInputSubsystem {
    pub fn new(client: &ShellClient) -> Result<Self> {
        let input_method = unsafe {
            xlib::XOpenIM(client.xdisplay, null_mut(), null_mut(), null_mut())
        };
        
        if input_method.is_null() {
            return Err(Error::PlatformApiFailed("cannot create input method"));
        }

        Ok(Self {
            input_method
        })
    }


    pub fn destroy(&self) {
        unsafe {
            xlib::XCloseIM(self.input_method);
        }
    }
}



impl X11SysRedrawSubsystem {
    pub fn new(client: &ShellClient, window_handle: X11WindowHandle) -> Result<Self> {
        let myself = Self::create(client, window_handle)?;
        myself.init_for_window(client, window_handle)?;
        Ok(myself)
    }


    pub fn destroy(&self, client: &ShellClient) -> Result<()> {
        client.connection.send_and_check_request(&xcb::sync::DestroyCounter {
            counter: self.sync_counter,
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy sync counter")))?;

        Ok(())
    }


    fn create(client: &ShellClient, window_handle: X11WindowHandle) -> Result<Self> {
        let sync_counter = client.connection.generate_id();
        let sync_value = xcb::sync::Int64 { hi: 0, lo: 0 };

        client.connection.send_and_check_request(&xcb::sync::CreateCounter {
            id: sync_counter,
            initial_value: sync_value,
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot create sync counter")))?;

        Ok(Self { sync_counter, sync_value })
    }


    fn init_for_window(&self, client: &ShellClient, window_handle: X11WindowHandle) -> Result<()> {
        use xcb::Xid;

        client.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: window_handle,
            property: client.atoms._NET_WM_SYNC_REQUEST_COUNTER,
            r#type: xcb::x::ATOM_CARDINAL,
            data: &[self.sync_counter.resource_id()]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot init sync counter")))?;

        Ok(())
    }


    /// Handles the data received with ClientMessage/_NET_WM_SYNC_REQUEST
    pub fn set_sync_value(&mut self, value: i64) {
        self.sync_value.lo = (value & 0xFF_FF_FF_FF) as u32;
        self.sync_value.hi = ((value >> 32) & 0xFF_FF_FF_FF) as i32;
    }


    fn get_sync_value(&self) -> i64 {
        (self.sync_value.hi as i64) << 32 + self.sync_value.lo as i64
    }


    fn increment_sync_value(&mut self) {
        self.set_sync_value(self.get_sync_value() + 1);
    }


    fn synchronise(&self, client: &ShellClient) -> Result<()> {
        client.connection.send_and_check_request(&xcb::sync::SetCounter {
            counter: self.sync_counter,
            value: self.sync_value
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set sync counter")))?;
        Ok(())
    }


    /// Forbids the shell to update the surface on the screen
    pub fn lock_surface(&mut self, client: &ShellClient) -> Result<()> {
        self.increment_sync_value();
        self.synchronise(client)
    }


    /// Allows the shell to update the surface on teh screen
    pub fn unlock_surface(&mut self, client: &ShellClient) -> Result<()> {
        self.increment_sync_value();
        self.synchronise(client)
    }
}



impl X11TextInputSubsystem {

    pub fn new(client: &ShellClient, window_handle: X11WindowHandle) -> Result<Self> {
        use xcb::Xid;

        if client.text_input_subsystem.is_none() {
            return Err(Error::InternalFailure);
        }

        let input_style = ffi::CString::new(xlib::XNInputStyle).unwrap();
        let client_window = ffi::CString::new(xlib::XNClientWindow).unwrap();
        let focus_window = ffi::CString::new(xlib::XNFocusWindow).unwrap();

        let input_method = client.text_input_subsystem.as_ref().unwrap().input_method;

        let xic = unsafe {
            xlib::XCreateIC(
                input_method,
                input_style.as_ptr(), xlib::XIMPreeditNothing | xlib::XIMStatusNothing,
                client_window.as_ptr(), window_handle.resource_id(),
                focus_window.as_ptr(), window_handle.resource_id(),
                null() as *const u8
            )
        };

        if xic.is_null() {
            return Err(Error::PlatformApiFailed("cannot create input context"));
        }

        Ok(X11TextInputSubsystem {
            input_context: xic,
            input: Vec::with_capacity(16),
            input_finished: false
        })
    }


    pub fn destroy(&self) {
        unsafe {
            xlib::XDestroyIC(self.input_context);
        }
    }

}
