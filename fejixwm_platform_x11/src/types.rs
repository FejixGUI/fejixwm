#![allow(non_snake_case)]

use crate::core;

use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};


/// Applications rarely have more than 4 windows.
pub type WindowStore<T> = HashMap<core::WindowId, T>;


pub struct WindowManager {
    pub(crate) name: String,
    
    pub(crate) connection: xcb::Connection,
    pub(crate) atoms: XAtoms,
    pub(crate) default_screen_number: i32,
    pub(crate) input_method: x11::xlib::XIM,
    
    pub(crate) windows: WindowStore<xcb::x::Window>,
    pub(crate) window_state_cache: WindowStore<WindowState>,
    pub(crate) smooth_redraw_drivers: WindowStore<WindowSmoothRedrawDriver>,
    pub(crate) input_drivers: WindowStore<WindowInputDriver>,
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

pub(crate) struct WindowInputDriver {
    pub(crate) input_context: x11::xlib::XIC,
    pub(crate) input: Vec<u8>,
    pub(crate) input_finished: bool,
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
