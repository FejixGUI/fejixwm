use crate::core::*;

use std::{
    ops::FnMut,
};


pub trait ShellMessageTrait : Sized {

    /// Returns `None` if the event is a global event
    fn get_window_id(&self) -> Option<WindowId>;


    fn is_global(&self) -> bool {
        self.get_window_id().is_none()
    }

}


pub enum Event {
    /// Sent by [ShellClientTrait::trigger_message]
    Trigger,

    Close,
    Resize { new_size: PixelSize },
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
impl<ShellClientT: ShellClientTrait, EventCallbackT> MessageCallback<ShellClientT> for EventCallbackT
where
    EventCallbackT: FnMut(Option<&ShellClientT::ShellMessage>, &mut ListeningSettings)
{}

// Make all closures that look like event handlers actual event handlers
impl<ShellClientT: ShellClientTrait, EventHandlerT> EventHandler<ShellClientT> for EventHandlerT
where
    EventHandlerT: FnMut(Event, Option<&mut ShellClientT::Window>)
{}



impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trigger => write!(f, "triggered"),
            Self::Close => write!(f, "closed"),
            Self::Resize { new_size } => write!(f, "resized to {new_size}"),
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