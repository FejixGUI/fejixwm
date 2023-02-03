use crate::core::*;

use std::{
    any::Any,
    ops::FnMut,
};


pub enum AnyEvent {
    GlobalEvent { event: GlobalEvent },
    WindowEvent { window_id: WindowId, event: WindowEvent }
}

pub enum GlobalEvent {

}

pub enum WindowEvent {
    Close,

    Resize(PixelSize),

    UserEvent { data: Box<dyn Any> }
}


pub trait EventHandler<WindowManagerT> : 'static + FnMut(&mut WindowManagerT, AnyEvent) {}

/// A better way to write `dyn EventHandler<..>`. This can be used a `Box<..>` type parameter.
pub type DynEventHandler<WindowManagerT> = dyn FnMut(&mut WindowManagerT, AnyEvent);

// Implement EventHandler for all FnMut that seem like event handlers
impl<WmT, FnT> EventHandler<WmT> for FnT where FnT: 'static + FnMut(&mut WmT, AnyEvent) {}


impl std::fmt::Display for AnyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowEvent { window_id, event } => 
                write!(f, "window id: {window_id}, event: {event}"),

            Self::GlobalEvent { event } =>
                write!(f, "global event: {event}"),
        }
    }
}


impl std::fmt::Display for WindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close => write!(f, "closed"),
            Self::Resize(size) => write!(f, "resized to {size}"),
            Self::UserEvent { .. } => write!(f, "user event")
        }
    }
}


impl std::fmt::Display for GlobalEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}