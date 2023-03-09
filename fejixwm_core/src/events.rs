use crate::core::*;

use std::{
    any::Any,
    ops::FnMut,
};


pub trait ShellMessageTrait : Sized {

    /// Returns `None` if the message is global
    fn get_window_id(&self)
        -> Option<WindowId>;


    fn is_global(&self)
        -> bool
    {
        self.get_window_id().is_none()
    }

}


pub enum Event {
    GlobalEvent(GlobalEvent),
    WindowEvent(WindowEvent),
    UserEvent(UserEvent),
}

pub enum GlobalEvent {

}

pub enum WindowEvent {
    Close,
    Resize { new_size: PixelSize },
}


pub struct UserEvent {
    pub data: Option<Box<dyn Any>>
}


pub enum ListeningBehavior {
    /// Makes [ShellClientTrait::listen_to_messages] handle the next event if available or None otherwise.
    Peek,

    /// Makes [ShellClientTrait::listen_to_messages] block its thread until any new events are received.
    Await,
}


pub struct ListeningSettings {
    pub behavior: ListeningBehavior,
    pub should_stop: bool,
}


/// No shell event is sent when the current behavior is [ListeningBehavior::Peek] but there are no events.
pub trait MessageCallback<ShellClientT: ShellClientTrait>
    : FnMut(Option<&ShellClientT::ShellMessage>, &mut ListeningSettings)
{}

/// No window is passed when the event is global
pub trait EventHandler<ShellClientT: ShellClientTrait>
    : FnMut(Event, Option<&mut ShellClientT::Window>)
{}


// Make all closures that look like message callbacks actual message callbacks
impl<ShellClientT: ShellClientTrait, MessageCallbackT> MessageCallback<ShellClientT> for MessageCallbackT
where
    MessageCallbackT: FnMut(Option<&ShellClientT::ShellMessage>, &mut ListeningSettings)
{}

// Make all closures that look like event handlers actual event handlers
impl<ShellClientT: ShellClientTrait, EventHandlerT> EventHandler<ShellClientT> for EventHandlerT
where
    EventHandlerT: FnMut(Event, Option<&mut ShellClientT::Window>)
{}



impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlobalEvent(event) => write!(f, "global event: {event}"),
            Self::WindowEvent(event) => write!(f, "window event: {event}"),
            Self::UserEvent(event) => write!(f, "user event: {event}"),
        }
    }
}


impl std::fmt::Display for GlobalEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}


impl std::fmt::Display for WindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close => write!(f, "closed"),
            Self::Resize { new_size } => write!(f, "resized to {new_size}"),
        }
    }
}


impl std::fmt::Display for UserEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_none() {
            write!(f, "None")
        } else {
            write!(f, "Some(..)")
        }
    }
}


impl Default for ListeningBehavior {
    fn default() -> Self {
        Self::Peek
    }
}


impl Default for ListeningSettings {
    fn default() -> Self {
        Self {
            behavior: ListeningBehavior::default(),
            should_stop: false,
        }
    }
}