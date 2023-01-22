use crate::core::{*, errors::{Result, Error}};
use crate::types::*;

use x11::xlib;
use xcb;

use std::{
    collections::HashMap,
    ptr::{null, null_mut}
};



impl WindowManager {
    pub fn new(info: &WindowManagerInfo) -> Result<Self> {
        let (connection, default_screen_number) = Self::connect()?;
        let input_method = Self::create_input_method(&connection)?;
        let atoms = XAtoms::intern_all(&connection)
            .or_else(|_| Err(Error::PlatformApiFailed("cannot get X atoms")))?;

        Ok(Self {
            name: info.name.clone(),

            connection,
            default_screen_number,
            atoms,
            input_method,

            windows: WindowStorage::new(),
            window_state_cache: WindowStorage::new(),
            smooth_redraw_drivers: WindowStorage::new(),
            text_input_drivers: WindowStorage::new(),
        })
    }


    fn destroy(&mut self) -> Result<()> {
        self.destroy_windows()?;
        self.detroy_input_method();
        Ok(())
    }


    fn register_window(&mut self, wid: WindowId, xwindow_id: xcb::x::Window) {
        self.windows.insert(wid, xwindow_id);
    }


    fn unregister_window(&mut self, wid: &WindowId) {
        self.windows.remove(&wid);
    }


    fn get_xwindow(&self, wid: &WindowId) -> Result<xcb::x::Window> {
        let xwindow = self.windows.get(&wid)
            .ok_or_else(|| Error::InvalidArgument)?;

        Ok(xwindow.clone())
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


    fn create_window(&mut self, info: &WindowInfo) -> Result<()> {
        let visual_info: WindowVisualInfo = self.create_visual_info(info)?;
        let wid = info.id;

        let xwindow = self.create_xwindow(info, &visual_info)?;
        self.register_window(wid, xwindow);
        self.set_xwindow_protocols(wid, info)?;
        self.init_window_drivers(wid, info.flags)?;
        self.create_window_canvas(wid, info)?;

        Ok(())
    }


    fn create_xwindow(&self, info: &WindowInfo, visual_info: &WindowVisualInfo) -> Result<xcb::x::Window> {
        let xwindow_id = self.connection.generate_id();

        let default_screen = self.get_default_screen();

        self.connection.send_and_check_request(&xcb::x::CreateWindow {
            wid: xwindow_id,
            parent: default_screen.root(),
            class: xcb::x::WindowClass::InputOutput,
            
            x: 0,
            y: 0,
            width: info.size.width as u16,
            height: info.size.height as u16,
            border_width: 0,
            
            // TODO Is screen depth important?
            depth: xcb::x::COPY_FROM_PARENT as u8,
            visual: visual_info.visualid,
            value_list: &[
                xcb::x::Cw::BackPixel(default_screen.black_pixel()),
                xcb::x::Cw::EventMask(xcb::x::EventMask::all()),
                xcb::x::Cw::Colormap(visual_info.colormap)
            ]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot create window")))?;

        Ok(xwindow_id)
    }


    fn destroy_xwindow(&mut self, wid: WindowId) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::DestroyWindow {
            window: self.windows[&wid]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy window")))
    }


    fn destroy_windows(&mut self) -> Result<()> {
        let window_ids = self.windows.keys().copied().collect::<Vec<u32>>();
        for wid in window_ids {
            self.destroy_window_drivers(wid)?;
            self.destroy_xwindow(wid)?;
            self.unregister_window(&wid);
        }

        Ok(())
    }


    fn create_xwindow_protocols_list(&self, window_info: &core::WindowInfo) -> Vec<xcb::x::Atom> {
        let mut protocols = vec![
            self.atoms.WM_DELETE_WINDOW
        ];

        if window_info.flags.contains(core::WindowFlags::SMOOTH_REDRAW) {
            protocols.push(self.atoms._NET_WM_SYNC_REQUEST);
        }

        protocols
    }

    
    fn set_xwindow_protocols(&self, wid: WindowId, window_info: &core::WindowInfo) -> Result<()> {
        let xwindow = self.get_xwindow(&wid)?;
        let protocols = self.create_xwindow_protocols_list(window_info);

        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: xwindow,
            property: self.atoms.WM_PROTOCOLS,
            r#type: xcb::x::ATOM_ATOM,
            data: protocols.as_slice()
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set WM protocols")))?;

        Ok(())
    }


    fn init_window_drivers(&mut self, wid: WindowId, flags: WindowFlags) -> Result<()> {
        self.init_window_smooth_redraw_driver(wid, flags)?;
        self.init_window_text_input_driver(wid, flags)?;
        Ok(())
    }


    fn destroy_window_drivers(&mut self, wid: WindowId) -> Result<()> {
        self.destroy_window_smooth_redraw_driver(wid)?;
        self.destroy_window_text_input_driver(wid);
        Ok(())
    }


    /// Returns Ok(()) if the flags forbid to initialise the driver
    fn init_window_smooth_redraw_driver(&mut self, wid: WindowId, flags: WindowFlags) -> Result<()> {
        if flags.contains(WindowFlags::SMOOTH_REDRAW) {
            self.smooth_redraw_drivers.insert(
                wid,WindowSmoothRedrawDriver::new(self, self.get_xwindow(&wid)?)?
            ).unwrap();
        }
        Ok(())
    }


    fn init_window_text_input_driver(&mut self, wid: WindowId, flags: WindowFlags) -> Result<()> {
        if flags.contains(WindowFlags::TEXT_INPUT) {
            self.text_input_drivers.insert(
                wid, WindowTextInputDriver::new(self)?
            ).unwrap();
        }
        Ok(())
    }


    fn destroy_window_smooth_redraw_driver(&mut self, wid: WindowId) -> Result<()> {
        if let Some(mut driver) = self.smooth_redraw_drivers.remove(&wid) {
            driver.destroy(self)?;
        }
        Ok(())
    }


    fn destroy_window_text_input_driver(&mut self, wid: WindowId) {
        if let Some(mut driver) = self.text_input_drivers.remove(&wid) {
            driver.destroy();
        }
    }


    fn get_default_visual_info(&self) -> WindowVisualInfo {
        let screen = self.get_default_screen();
        
        WindowVisualInfo {
            visualid: screen.root_visual(),
            colormap: screen.default_colormap(),
        }
    }


    fn create_visual_info(&mut self, window_info: &WindowInfo) -> Result<WindowVisualInfo> {
        match window_info.canvas_info {
            CanvasInfo::None => Ok(self.get_default_visual_info()),

            // TODO implement graphics APIs
            _ => Err(Error::UnsupportedFeature),
        }
    }


    fn create_window_canvas(&mut self, wid: WindowId, info: &WindowInfo) -> Result<()> {
        match info.canvas_info {
            CanvasInfo::None => Ok(()),

            // TODO implement graphics APIs
            _ => Err(Error::UnsupportedFeature)
        }
    }

}


impl Drop for WindowManager {
    fn drop(&mut self) {
        self.destroy().unwrap();
    }
}



impl WindowSmoothRedrawDriver {
    pub fn new(wm: &WindowManager, window: xcb::x::Window) -> Result<Self> {
        let myself = Self::create_self(wm)?;
        myself.set_window_counter(wm, window)?;
        Ok(myself)
    }


    pub fn destroy(&mut self, wm: &WindowManager) -> Result<()> {
        self.destroy_counter(wm)
    }


    pub fn lock(&mut self, wm: &WindowManager) -> Result<()> {
        self.increment();
        self.sync(wm)
    }


    pub fn unlock(&mut self, wm: &WindowManager) -> Result<()> {
        self.increment();
        self.sync(wm)
    }


    /// Must be called on sync request event
    pub fn set_sync_value(&mut self, value: i64) {
        self.sync_value.lo = (value & 0xFF_FF_FF_FF) as u32;
        self.sync_value.hi = ((value >> 32) & 0xFF_FF_FF_FF) as i32;
    }


    fn get_sync_value(&self) -> i64 {
        (self.sync_value.hi as i64) << 32 + self.sync_value.lo as i64
    }


    fn create_self(wm: &WindowManager) -> Result<Self> {
        let sync_counter = wm.connection.generate_id();
        let sync_value = xcb::sync::Int64 { hi: 0, lo: 0 };

        wm.connection.send_and_check_request(&xcb::sync::CreateCounter {
            id: sync_counter,
            initial_value: sync_value,
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot create sync counter")))?;

        Ok(Self { sync_counter, sync_value })
    }


    fn destroy_counter(&mut self, wm: &WindowManager) -> Result<()> {
        wm.connection.send_and_check_request(&xcb::sync::DestroyCounter {
            counter: self.sync_counter,
        })
        .and_then(|_| Ok(()))
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy counter")))
    }


    fn set_window_counter(&self, wm: &WindowManager, window: xcb::x::Window) -> Result<()> {
        use xcb::Xid;

        wm.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window,
            property: wm.atoms._NET_WM_SYNC_REQUEST_COUNTER,
            r#type: xcb::x::ATOM_CARDINAL,
            data: &[self.sync_counter.resource_id()]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot init sync counter")))
        .and_then(|_| Ok(()))
    }


    fn sync(&mut self, wm: &WindowManager) -> Result<()> {
        wm.connection.send_and_check_request(&xcb::sync::SetCounter {
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



impl WindowTextInputDriver {
    pub fn new(wm: &WindowManager) -> Result<Self> {
        let xic = unsafe { xlib::XCreateIC(wm.input_method) };
        if xic.is_null() {
            return Err(Error::PlatformApiFailed("cannot create input context"));
        }

        Ok(Self {
            input_context: xic,
            input: Vec::with_capacity(16),
            input_finished: false
        })
    }


    pub fn destroy(&mut self) {
        unsafe {
            xlib::XDestroyIC(self.input_context);
        }
    }


    pub fn handle_key_event(&self, event: &xcb::x::KeyPressEvent) -> Result<()> {
        // TODO
        // let event = xlib::XKeyPressedEvent {
        //     type_ = xlib::KeyPress,
        //     display = self.
        // }

        Ok(())
    }
}


impl WindowManagerTrait for WindowManager {
    fn new(info: &WindowManagerInfo) -> Result<Self> {
        Self::new(info)
    }

    fn new_window(&mut self, info: &WindowInfo) -> Result<()> {
        self.create_window(info)
    }

    fn run<EventHandlerT : events::EventHandler>(&mut self) {
        todo!()
    }
}


impl interface::window_manip::WmVisibilityController for WindowManager {
    fn set_visible(&mut self, wid: WindowId, visible: bool) -> Result<()> {
        if visible {
            self.connection.send_and_check_request(&xcb::x::MapWindow {
                window: self.get_xwindow(&wid)?
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot map window")))?
        } else {
            self.connection.send_and_check_request(&xcb::x::UnmapWindow {
                window: self.get_xwindow(&wid)?
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot unmap window")))?
        }

        Ok(())
    }
}


impl interface::window_manip::WmTitleController for WindowManager {
    fn set_title(&mut self, wid: WindowId, title: &str) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: self.get_xwindow(&wid)?,
            property: self.atoms._NET_WM_NAME,
            r#type: self.atoms.UTF8_STRING,
            data: title.as_bytes()
        }) 
        .or_else(|_| Err(Error::PlatformApiFailed("failed to set title")))      
    }
}