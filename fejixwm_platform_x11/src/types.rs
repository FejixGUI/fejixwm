#![allow(non_snake_case)]

use crate::core;

use std::{
    rc::Rc,
    cell::RefCell, collections::HashMap,
};


pub struct App {
    pub(crate) app: Rc<RefCell<PlatformApp>>,
}

pub struct AppRef {
    pub(crate) app: Rc<RefCell<PlatformApp>>,
}


pub struct Window {
    pub(crate) app: AppRef,

    pub(crate) id: core::WindowId,

    pub(crate) xid: xcb::x::Window,

    pub(crate) smooth_redraw_driver: Option<WindowSmoothRedrawDriver>,
    pub(crate) input_driver: Option<WindowInputDriver>,
}

pub struct WindowInternalVisualData {
    pub(crate) visualid: xcb::x::Visualid,
    pub(crate) colormap: xcb::x::Colormap,
}

pub struct PlatformApp {
    pub(crate) name: String,
    pub(crate) window_ids: HashMap<core::WindowId, xcb::x::Window>,

    pub(crate) xdisplay: x11::xlib::Display,
    pub(crate) xdefault_screen: u32,

    pub(crate) conection: xcb::Connection,

    pub(crate) atoms: Atoms,
    pub(crate) input_method: x11::xlib::XIM
}

pub(crate) struct WindowSmoothRedrawDriver {
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
        WM_PROTOCOLS => b"WM_PROTOCOLS",
        WM_DELETE_WINDOW => b"WM_DELETE_WINDOW",

        _NET_WM_NAME => b"_NET_WM_NAME",
        UTF8_STRING => b"UTF8_STRING",

        _NET_WM_SYNC_REQUEST => b"_NET_WM_SYNC_REQUEST",
        _NET_WM_SYNC_REQUEST_COUNTER => b"_NET_WM_SYNC_REQUEST_COUNTER",
    }
}
