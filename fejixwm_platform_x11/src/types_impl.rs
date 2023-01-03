use crate::core::{self, Error, Result, traits::*};
use crate::types::*;

use x11::xlib;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    cell::RefCell,
    ptr::{null, null_mut}
};


pub struct PlatformApi;


impl PlatformApiTrait for PlatformApi {
    type App = App;
    type AppRef = AppRef;
    type Window = Window;
    type WindowInternalVisualData = WindowInternalVisualData;
}



impl AppTrait for App {

    type PlatformApi = PlatformApi;

    fn new(name: String) -> core::Result<Self> {
        Ok(Self {
            app: Arc::new(PlatformApp::new(name)?)
        })
    }

    fn get_ref(&self) -> AppRef {
        AppRef { app: Arc::clone(&self.app) }
    }

    fn run(&self, event_handler: core::events::EventHandler) {
        
    }

}



impl AppRefTrait for AppRef {

    type PlatformApi = PlatformApi;

}


impl Clone for AppRef {

    fn clone(&self) -> Self {
        Self { app: Arc::clone(&self.app) }
    }

}


impl WindowTrait for Window {

    type PlatformApi = PlatformApi;

    fn new_internal(
        app: AppRef,
        params: core::WindowParams,
        visual_data: WindowInternalVisualData
    ) -> core::Result<Self> {
        unimplemented!()
    }

    fn get_app(&self) -> AppRef {
        self.app.clone()
    }

    fn get_id(&self) -> core::WindowId {
        self.id
    }

    fn get_size(&self) -> core::Result<core::PixelSize> {
        Ok(self.size.clone())
    }

}



impl PlatformApp {
    pub fn new(name: String) -> core::Result<Self> {
        let (connection, default_screen) = Self::connect()?;
        let input_method = Self::create_input_method(&connection)?;
        let atoms = Atoms::intern_all(&connection)
            .or_else(|_| Err(Error::PlatformApiFailed("cannot get X atoms")))?;

        Ok(Self {
            name,
            window_ids: Mutex::new(HashMap::new()),
            connection,
            default_screen,
            atoms,
            input_method
        })
    }


    fn connect() -> Result<(xcb::Connection, i32)> {
        xcb::Connection::connect_with_xlib_display()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot connect to Xorg")))
    }


    fn create_input_method(connection: &xcb::Connection) -> Result<xlib::XIM> {
        let im = unsafe {
            xlib::XOpenIM(connection.get_raw_dpy(), null_mut(), null_mut(), null_mut())
        };
        
        if im.is_null() {
            return Err(Error::PlatformApiFailed("cannot create input method"));
        }

        return Ok(im)
    }


    fn detroy_input_method(&self) {
        unsafe {
            xlib::XCloseIM(self.input_method);
        }
    }
}


impl Drop for PlatformApp {
    fn drop(&mut self) {
        self.detroy_input_method();
    }
}