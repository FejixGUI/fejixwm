use crate::core::{*, errors::{Result, Error}};
use crate::types::*;

use x11::xlib;
use xcb;

use std::{
    collections::HashMap,
    ptr::{null, null_mut},
    ffi
};


mod _impl_smooth_redraw_driver {
    use super::*;

    impl WmSmoothRedrawDriver for WindowManager {
        fn new_driver(&mut self, wid: WindowId) -> Result<()> {
            let driver = self.create_driver()?;
            self.smooth_redraw_drivers.insert(wid, driver);
            self.init_driver_for_window(wid,)?;
            Ok(())
        }

        fn drop_driver(&mut self, wid: WindowId) -> Result<()> {
            if let Some(driver) = self.smooth_redraw_drivers.remove(&wid) {
                self.destroy_driver(driver)?;
            }
            Ok(())
        }

        fn lock(&mut self, wid: WindowId) -> Result<()> {
            self.increment_sync_value(wid);
            self.sync_counter(wid)
        }

        fn unlock(&mut self, wid: WindowId) -> Result<()> {
            // The unlocking procedure is the same as locking
            self.lock(wid)
        }

        fn update_sync_value(&mut self, wid: WindowId, value: i64) -> Result<()> {
            self.set_sync_value(wid, value);
            Ok(())
        }
    }


    impl WindowManager {
        fn create_driver(&self) -> Result<WindowSmoothRedrawDriver> {
            let sync_counter = self.connection.generate_id();
            let sync_value = xcb::sync::Int64 { hi: 0, lo: 0 };

            self.connection.send_and_check_request(&xcb::sync::CreateCounter {
                id: sync_counter,
                initial_value: sync_value,
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot create sync counter")))?;

            Ok(WindowSmoothRedrawDriver { sync_counter, sync_value })
        }


        fn destroy_driver(&self, driver: WindowSmoothRedrawDriver) -> Result<()> {
            self.connection.send_and_check_request(&xcb::sync::DestroyCounter {
                counter: driver.sync_counter,
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy counter")))
        }


        fn get_driver<'a>(&'a self, wid: WindowId) -> &'a WindowSmoothRedrawDriver {
            self.smooth_redraw_drivers.get(&wid).unwrap()
        }


        fn get_driver_mut<'a>(&'a mut self, wid: WindowId) -> &'a mut WindowSmoothRedrawDriver {
            self.smooth_redraw_drivers.get_mut(&wid).unwrap()
        }


        fn init_driver_for_window(&self, wid: WindowId) -> Result<()> {
            use xcb::Xid;

            let driver = self.get_driver(wid);

            self.connection.send_and_check_request(&xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: self.get_window_handle(&wid)?,
                property: self.atoms._NET_WM_SYNC_REQUEST_COUNTER,
                r#type: xcb::x::ATOM_CARDINAL,
                data: &[driver.sync_counter.resource_id()]
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot init sync counter")))
            .and_then(|_| Ok(()))
        }


        fn set_sync_value(&mut self, wid: WindowId, value: i64) {
            let driver = self.get_driver_mut(wid);
            driver.sync_value.lo = (value & 0xFF_FF_FF_FF) as u32;
            driver.sync_value.hi = ((value >> 32) & 0xFF_FF_FF_FF) as i32;
        }


        fn get_sync_value(&self, wid: WindowId) -> i64 {
            let driver = self.get_driver(wid);
            (driver.sync_value.hi as i64) << 32 + driver.sync_value.lo as i64
        }


        /// Synchronise the counter
        fn sync_counter(&self, wid: WindowId) -> Result<()> {
            let driver = self.get_driver(wid);
            self.connection.send_and_check_request(&xcb::sync::SetCounter {
                counter: driver.sync_counter,
                value: driver.sync_value
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot set sync counter")))
        }


        fn increment_sync_value(&mut self, wid: WindowId) {
            let mut value = self.get_sync_value(wid);
            value += 1;
            self.set_sync_value(wid, value);
        }
    }

}

impl WindowManagerTrait for WindowManager {
    fn new(info: &WindowManagerInfo) -> Result<Self> {
        Self::new(info)
    }

    fn new_window(&mut self, info: &WindowInfo) -> Result<()> {
        self.create_window(info)
    }

    fn drop_window(&mut self, wid: WindowId) -> Result<()> {
        self.destroy_window(wid)
    }

    fn run<EventHandlerT : events::EventHandler>(&mut self) {
        todo!()
    }
}


impl interface::window_manip::WmVisibilityController for WindowManager {
    fn set_visible(&mut self, wid: WindowId, visible: bool) -> Result<()> {
        if visible {
            self.connection.send_and_check_request(&xcb::x::MapWindow {
                window: self.get_window_handle(&wid)?
            })
            .or_else(|_| Err(Error::PlatformApiFailed("cannot map window")))?
        } else {
            self.connection.send_and_check_request(&xcb::x::UnmapWindow {
                window: self.get_window_handle(&wid)?
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
            window: self.get_window_handle(&wid)?,
            property: self.atoms._NET_WM_NAME,
            r#type: self.atoms.UTF8_STRING,
            data: title.as_bytes()
        }) 
        .or_else(|_| Err(Error::PlatformApiFailed("failed to set title")))      
    }
}