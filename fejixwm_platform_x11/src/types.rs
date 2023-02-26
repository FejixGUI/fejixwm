#![allow(non_snake_case)]

pub(crate) use crate::core::{
    errors::*,
    events::{
        Event, ShellMessageTrait, MessageCallback, EventHandler, EventListeningBehavior, EventListeningSettings,
    },
    *
};

pub(crate) use x11::xlib;
pub(crate) use xcb;

pub(crate) use std::{
    ptr::{null, null_mut},
    ffi,
};



pub(crate) type X11WindowHandle = xcb::x::Window;

pub(crate) fn window_handle_to_id(handle: X11WindowHandle) -> WindowId {
    use xcb::Xid;
    handle.resource_id() as usize
}

pub struct ShellClient {
    pub(crate) connection: xcb::Connection,
    pub(crate) xdisplay: *mut xlib::Display,
    pub(crate) atoms: X11Atoms,
    pub(crate) default_screen_number: i32,
    pub(crate) class_name: String,

    /// Passed to X11 functions that take a window handle but have no practical reason for doing so
    pub(crate) fake_window_handle: X11WindowHandle,

    pub(crate) text_input_subsystem: Option<X11GlobalTextInputSubsystem>,
}

pub struct Window {
    pub(crate) handle: X11WindowHandle,
    pub(crate) state: X11WindowState,
    pub(crate) text_input: Option<X11TextInputSubsystem>,
    pub(crate) sys_redraw: Option<X11SysRedrawSubsystem>
}


pub struct ShellMessage {
    pub(crate) event: xcb::Event,
    pub(crate) is_global: bool,
    pub(crate) window_handle: Option<X11WindowHandle>,
}


xcb::atoms_struct! {
    pub(crate) struct X11Atoms {
        pub WM_PROTOCOLS => b"WM_PROTOCOLS",
        pub WM_DELETE_WINDOW => b"WM_DELETE_WINDOW",
        pub _NET_WM_PING => b"_NET_WM_PING",

        pub _NET_WM_NAME => b"_NET_WM_NAME",
        pub UTF8_STRING => b"UTF8_STRING",

        pub _NET_WM_SYNC_REQUEST => b"_NET_WM_SYNC_REQUEST",
        pub _NET_WM_SYNC_REQUEST_COUNTER => b"_NET_WM_SYNC_REQUEST_COUNTER",
    }
}



pub(crate) struct X11WindowState {
    pub size: PixelSize,
}

pub(crate) struct X11WindowVisualInfo {
    pub visualid: xcb::x::Visualid,
    pub colormap: xcb::x::Colormap,
    pub color_depth: u8,
}


pub(crate) struct X11SysRedrawSubsystem {
    pub sync_counter: xcb::sync::Counter,
    pub sync_value: xcb::sync::Int64,
}

pub(crate) struct X11GlobalTextInputSubsystem {
    pub input_method: xlib::XIM,
}

pub(crate) struct X11TextInputSubsystem {
    pub input_context: xlib::XIC,
    pub input: Vec<u8>,
    pub input_finished: bool,
}