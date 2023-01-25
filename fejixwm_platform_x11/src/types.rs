#![allow(non_snake_case)]

pub(crate) use crate::core::*;

pub(crate) use x11::xlib;
pub(crate) use xcb;

pub(crate) use std::{
    collections::HashMap,
    ptr::{null, null_mut},
    ffi
};


/// Applications rarely have more than 4 windows.
pub type WindowStorage<T> = HashMap<WindowId, T>;

pub(crate) type WindowHandle = xcb::x::Window;


pub struct WindowManager {
    pub(crate) name: String,
    pub(crate) should_stop: bool,
    pub(crate) event_handler: Option<Box<events::DynEventHandler<Self>>>,

    pub(crate) connection: xcb::Connection,
    pub(crate) atoms: XAtoms,
    pub(crate) default_screen_number: i32,
    pub(crate) input_method: x11::xlib::XIM,
    
    pub(crate) window_handles: WindowStorage<WindowHandle>,
    pub(crate) window_state_cache: WindowStorage<WindowState>,
    pub(crate) smooth_redraw_drivers: WindowStorage<WindowSmoothRedrawDriver>,
    pub(crate) text_input_drivers: WindowStorage<WindowTextInputDriver>,
}


xcb::atoms_struct! {
    pub(crate) struct XAtoms {
        pub WM_PROTOCOLS => b"WM_PROTOCOLS",
        pub WM_DELETE_WINDOW => b"WM_DELETE_WINDOW",

        pub _NET_WM_NAME => b"_NET_WM_NAME",
        pub UTF8_STRING => b"UTF8_STRING",

        pub _NET_WM_SYNC_REQUEST => b"_NET_WM_SYNC_REQUEST",
        pub _NET_WM_SYNC_REQUEST_COUNTER => b"_NET_WM_SYNC_REQUEST_COUNTER",
    }
}



pub(crate) struct WindowState {
    pub(crate) size: core::PixelSize,
}

pub(crate) struct WindowVisualInfo {
    pub(crate) visualid: xcb::x::Visualid,
    pub(crate) colormap: xcb::x::Colormap,
}


pub(crate) struct WindowSmoothRedrawDriver {
    pub(crate) sync_counter: xcb::sync::Counter,
    pub(crate) sync_value: xcb::sync::Int64,
}

pub(crate) trait WmSmoothRedrawDriver {
    fn new_driver(&mut self, wid: WindowId) -> Result<()>;

    /// Does nothing if no driver was created for the window
    fn drop_driver(&mut self, wid: WindowId) -> Result<()>;

    fn lock(&mut self, wid: WindowId) -> Result<()>;
    fn unlock(&mut self, wid: WindowId) -> Result<()>;

    /// Must be called on sync request event
    fn update_sync_value(&mut self, wid: WindowId, value: i64) -> Result<()>;
}

pub(crate) struct WindowTextInputDriver {
    pub(crate) input_context: x11::xlib::XIC,
    pub(crate) input: Vec<u8>,
    pub(crate) input_finished: bool,
}


pub(crate) trait WmTextInputDriver {
    fn new_driver(&mut self, wid: WindowId) -> Result<()>;

    /// Does nothing if no driver was created for the window
    fn drop_driver(&mut self, wid: WindowId) -> Result<()>;

    // TODO Finish text input driver interface

    // fn handle_key_event(&self, event: &xcb::x::KeyPressEvent) -> Result<()> {
    //     // TODO
    //     // let event = xlib::XKeyPressedEvent {
    //     //     type_ = xlib::KeyPress,
    //     //     display = self.
    //     // }

    //     Ok(())
    // }
}