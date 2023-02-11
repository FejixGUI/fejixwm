use crate::core::*;

use std::{
    any::Any,
    ops::FnMut,
};


pub enum Event {
    Close,
    Resize { new_size: PixelSize },
    UserEvent { data: Box<dyn Any> }
}



pub trait EventHandler<ShellClientT: ShellClientTrait>
    : 'static + FnMut(&ShellClientT, &mut ShellClientT::Window, Event)
{}

// Make all closures that look like event handlers actual event handlers
impl<ShellClientT: ShellClientTrait, EventHandlerT> EventHandler<ShellClientT> for EventHandlerT
where
    EventHandlerT: 'static + FnMut(&ShellClientT, &mut ShellClientT::Window, Event)
{}


impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close => write!(f, "closed"),
            Self::Resize { new_size } => write!(f, "resized to {new_size}"),
            Self::UserEvent { data } => write!(f, "user-defined event")
        }
    }
}
