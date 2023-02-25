use crate::core::*;

use std::{
    ops::FnMut,
};


pub trait SystemEventTrait {

    /// Returns `None` if the event if a global event
    fn get_window_id(&self) -> Option<WindowId>;

}


pub enum Event {
    /// Sent by [ShellClientTrait::wakeup] in order to interrupt waiting for events
    Wakeup,

    Close,
    Resize { new_size: PixelSize },
}


pub enum EventListenBehavior {
    /// Makes [ShellClientTrait::listen_to_events] handle the next event if available or None otherwise.
    GetNextEvent,

    /// Makes [ShellClientTrait::listen_to_events] return as soon as possible.
    Quit,

    /// Makes [ShellClientTrait::listen_to_events] block its thread until any new events are received.
    WaitForEvents,
}


pub struct EventListenSettings {
    pub behavior: EventListenBehavior
}


pub trait EventCallback<ShellClientT: ShellClientTrait>
    : FnMut(Option<&ShellClientT::SystemEvent>, &mut EventListenSettings) -> EventListenBehavior
{}

// Make all closures that look like event handlers actual event handlers
impl<ShellClientT: ShellClientTrait, EventHandlerT> EventCallback<ShellClientT> for EventHandlerT
where
    EventHandlerT: FnMut(Option<&ShellClientT::SystemEvent>, &mut EventListenSettings) -> EventListenBehavior
{}



impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wakeup => write!(f, "woke up"),
            Self::Close => write!(f, "closed"),
            Self::Resize { new_size } => write!(f, "resized to {new_size}"),
        }
    }
}
