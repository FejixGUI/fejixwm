use crate::core::{self, Error, Result, traits::*};
use crate::types::*;

use x11::xlib;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
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
        todo!()
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
        Self::new(app, params, visual_data)
    }

    fn get_app(&self) -> AppRef {
        self.app.clone()
    }

    fn get_id(&self) -> core::WindowId {
        self.id
    }

    fn get_size(&self) -> core::PixelSize {
        self.size.clone()
    }

}



impl PlatformApp {
    pub(crate) fn new(name: String) -> core::Result<Self> {
        let (connection, default_screen_number) = Self::connect()?;
        let input_method = Self::create_input_method(&connection)?;
        let atoms = Atoms::intern_all(&connection)
            .or_else(|_| Err(Error::PlatformApiFailed("cannot get X atoms")))?;

        Ok(Self {
            name,
            window_ids: Mutex::new(HashMap::new()),
            connection,
            default_screen_number,
            atoms,
            input_method
        })
    }


    pub(crate) fn add_window_id(&self, window: xcb::x::Window, id: core::WindowId) -> Result<()> {
        let mut window_ids = self.window_ids.try_lock()
            .or_else(|_| Err(Error::InternalLogicFailed))?;
        
        window_ids.insert(window, id);

        Ok(())
    }


    pub(crate) fn remove_window_id(&self, window: &xcb::x::Window) -> Result<()> {
        let mut window_ids = self.window_ids.try_lock()
            .or_else(|_| Err(Error::InternalLogicFailed))?;
        
        window_ids.remove(&window);

        Ok(())
    }


    pub(crate) fn get_default_screen(&self) -> &xcb::x::Screen {
        self.connection.get_setup().roots().nth(self.default_screen_number as usize).unwrap()
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


    fn detroy_input_method(&mut self) {
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


impl Window {
    fn new(
        app: AppRef,
        params: core::WindowParams,
        visual_data: WindowInternalVisualData
    ) -> core::Result<Self> {

        let xid = Self::create_window(&app, &params, visual_data)?;
        Self::set_window_protocols(&app, xid, &params)?;
        app.app.add_window_id(xid, params.id)?;

        let mut myself = Self {
            app,
            id: params.id,
            size: params.size.clone(),
            xid,
            smooth_redraw_driver: None,
            input_driver: None
        };

        myself.init_drivers(&params)?;

        Ok(myself)
    
    }


    pub(crate) fn get_smooth_redraw_driver_mut(&self) -> Result<MutexGuard<WindowSmoothRedrawDriver>> {
        if self.smooth_redraw_driver.is_none() {
            return Err(Error::InternalLogicFailed)
        }

        self.smooth_redraw_driver.as_ref().unwrap().try_lock()
            .or_else(|_| Err(Error::InternalLogicFailed))
    }


    fn create_window(
        app: &AppRef,
        params: &core::WindowParams,
        visual_data: WindowInternalVisualData
    ) -> Result<xcb::x::Window> {
        let xid = app.app.connection.generate_id();

        let default_screen = app.app.get_default_screen();
        let default_parent_window = default_screen.root();

        app.app.connection.send_and_check_request(&xcb::x::CreateWindow {
            wid: xid,
            parent: default_parent_window,
            class: xcb::x::WindowClass::InputOutput,
            
            x: 0,
            y: 0,
            width: params.size.width as u16,
            height: params.size.height as u16,
            border_width: 0,
            
            // TODO Is screen depth important?
            depth: xcb::x::COPY_FROM_PARENT as u8,
            visual: visual_data.visualid,
            value_list: &[
                xcb::x::Cw::BackPixel(default_screen.black_pixel()),
                xcb::x::Cw::EventMask(xcb::x::EventMask::all()),
                xcb::x::Cw::Colormap(visual_data.colormap)
            ]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot create window")))?;

        Ok(xid)
    }


    fn destroy(&self) -> Result<()> {
        self.app.app.remove_window_id(&self.xid)?;
        self.destroy_drivers()?;
        self.destroy_window()?;
        Ok(())
    }

    fn destroy_window(&self) -> Result<()> {
        self.app.app.connection.send_and_check_request(&xcb::x::DestroyWindow {
            window: self.xid
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot delete window")))
        .and_then(|_| Ok(()))
    }


    fn init_drivers(&mut self, params: &core::WindowParams) -> Result<()> {
        if params.flags.contains(core::WindowFlags::SMOOTH_REDRAW) {
            self.smooth_redraw_driver = Some(Mutex::new(WindowSmoothRedrawDriver::new(&self.app, self.xid)?));
        }

        Ok(())
    }


    fn destroy_drivers(&self) -> Result<()> {
        if self.smooth_redraw_driver.is_some() {
            let mut driver = self.get_smooth_redraw_driver_mut()?;
            driver.destroy_counter()?;
        }

        Ok(())
    }


    fn set_window_protocols(
        app: &AppRef,
        window: xcb::x::Window,
        params: &core::WindowParams
    ) -> Result<()> {

        let mut protocols = vec![
            app.app.atoms.WM_DELETE_WINDOW
        ];

        if params.flags.contains(core::WindowFlags::SMOOTH_REDRAW) {
            protocols.push(app.app.atoms._NET_WM_SYNC_REQUEST);
        }

        app.app.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window,
            property: app.app.atoms.WM_PROTOCOLS,
            r#type: xcb::x::ATOM_ATOM,
            data: protocols.as_slice()
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set WM protocols")))?;

        Ok(())
    }
}


impl Drop for Window {
    fn drop(&mut self) {
        self.destroy().unwrap();
    }
}


impl WindowSmoothRedrawDriver {
    pub fn new(
        app: &AppRef,
        window: xcb::x::Window
    ) -> Result<Self> {
        let myself = Self::create_counter(app)?;
        myself.set_window_counter(app, window)?;
        Ok(myself)
    }


    pub fn lock(&mut self) -> Result<()> {
        self.increment();
        self.sync()
    }

    pub fn unlock(&mut self) -> Result<()> {
        self.increment();
        self.sync()
    }


    pub fn set_sync_value(&mut self, value: i64) {
        self.sync_value.lo = (value & 0xFF_FF_FF_FF) as u32;
        self.sync_value.hi = ((value >> 32) & 0xFF_FF_FF_FF) as i32;
    }


    fn get_sync_value(&self) -> i64 {
        (self.sync_value.hi as i64) << 32 + self.sync_value.lo as i64
    }


    fn create_counter(app: &AppRef) -> Result<Self> {
        let sync_counter = app.app.connection.generate_id();
        let sync_value = xcb::sync::Int64 { hi: 0, lo: 0 };

        app.app.connection.send_and_check_request(&xcb::sync::CreateCounter {
            id: sync_counter,
            initial_value: sync_value,
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot create sync counter")))?;

        Ok(Self { app: app.clone(), sync_counter, sync_value })
    }


    fn destroy_counter(&mut self) -> Result<()> {
        self.app.app.connection.send_and_check_request(&xcb::sync::DestroyCounter {
            counter: self.sync_counter,
        })
        .and_then(|_| Ok(()))
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy counter")))
    }


    fn set_window_counter(
        &self,
        app: &AppRef,
        window: xcb::x::Window
    ) -> Result<()> {
        use xcb::Xid;

        app.app.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window,
            property: app.app.atoms._NET_WM_SYNC_REQUEST_COUNTER,
            r#type: xcb::x::ATOM_CARDINAL,
            data: &[self.sync_counter.resource_id()]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot init sync counter")))
        .and_then(|_| Ok(()))
    }


    fn sync(&mut self) -> Result<()> {
        self.app.app.connection.send_and_check_request(&xcb::sync::SetCounter {
            counter: self.sync_counter,
            value: self.sync_value
        })
        .and_then(|_| Ok(()))
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set sync counter")))
    }


    fn increment(&mut self) {
        let mut value = self.get_sync_value();
        value += 1;
        self.set_sync_value(value);
    }
}


impl Drop for WindowSmoothRedrawDriver {
    fn drop(&mut self) {
        self.destroy_counter().unwrap();
    }
}