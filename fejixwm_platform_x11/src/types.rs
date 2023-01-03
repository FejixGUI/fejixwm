#![allow(non_snake_case)]

use crate::core;

use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};


pub struct App {
    pub(crate) app: Arc<PlatformApp>,
}

pub struct AppRef {
    pub(crate) app: Arc<PlatformApp>,
}


pub struct Window {
    pub(crate) app: AppRef,

    pub(crate) size: Mutex<core::PixelSize>,

    pub(crate) id: core::WindowId,

    pub(crate) xid: xcb::x::Window,

    /// Must be accessed only inside app.run()
    pub(crate) smooth_redraw_driver: Option<Mutex<WindowSmoothRedrawDriver>>,
    /// Must be accessed only inside app.run()
    pub(crate) input_driver: Option<Mutex<WindowInputDriver>>,
}

pub struct WindowInternalVisualData {
    pub(crate) visualid: xcb::x::Visualid,
    pub(crate) colormap: xcb::x::Colormap,
}

pub(crate) struct PlatformApp {
    pub(crate) name: String,
    pub(crate) window_ids: Mutex<HashMap<xcb::x::Window, core::WindowId>>,

    pub(crate) connection: xcb::Connection,
    pub(crate) default_screen_number: i32,

    pub(crate) atoms: Atoms,
    pub(crate) input_method: x11::xlib::XIM
}

pub(crate) struct WindowSmoothRedrawDriver {
    pub(crate) app: AppRef,
    pub(crate) sync_counter: xcb::sync::Counter,
    pub(crate) sync_value: xcb::sync::Int64,
}

pub(crate) struct WindowInputDriver {
    pub(crate) input_context: x11::xlib::XIC,
    pub(crate) input: Vec<u8>,
    pub(crate) input_finished: bool,
}


xcb::atoms_struct! {
    pub(crate) struct Atoms {
        pub WM_PROTOCOLS => b"WM_PROTOCOLS",
        pub WM_DELETE_WINDOW => b"WM_DELETE_WINDOW",

        pub _NET_WM_NAME => b"_NET_WM_NAME",
        pub UTF8_STRING => b"UTF8_STRING",

        pub _NET_WM_SYNC_REQUEST => b"_NET_WM_SYNC_REQUEST",
        pub _NET_WM_SYNC_REQUEST_COUNTER => b"_NET_WM_SYNC_REQUEST_COUNTER",
    }
}
