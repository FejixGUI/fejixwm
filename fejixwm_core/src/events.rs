use crate::core::*;

use std::{
    ops::FnMut,
};


pub trait ShellEventTrait : Sized {

    /// Returns `None` if the event is a global event
    fn get_window_id(&self) -> Option<WindowId>;

}


pub enum Event {
    /// Sent by [ShellClientTrait::wakeup] in order to interrupt waiting for events
    Wakeup,

    Close,
    Resize { new_size: PixelSize },
}


pub enum EventListeningBehavior {
    /// Makes [ShellClientTrait::listen_to_events] handle the next event if available or None otherwise.
    GetNextEvent,

    /// Makes [ShellClientTrait::listen_to_events] return as soon as possible.
    Quit,

    /// Makes [ShellClientTrait::listen_to_events] block its thread until any new events are received.
    WaitForEvents,
}


pub struct EventListeningSettings {
    pub behavior: EventListeningBehavior
}


/// No shell event is sent when the current behavior is [EventListeningBehavior::GetNextEvent] but there are no events.
pub trait EventCallback<ShellClientT: ShellClientTrait>
    : FnMut(Option<&ShellClientT::ShellEvent>, &mut EventListeningSettings)
{}

/// No window is passed when the event is global
pub trait EventHandler<ShellClientT: ShellClientTrait>
    : FnMut(Event, Option<&mut ShellClientT::Window>)
{}

// Make all closures that look like event callbacks actual event callbacks
impl<ShellClientT: ShellClientTrait, EventCallbackT> EventCallback<ShellClientT> for EventCallbackT
where
    EventCallbackT: FnMut(Option<&ShellClientT::ShellEvent>, &mut EventListeningSettings)
{}

// Make all closures that look like event handlers actual event handlers
impl<ShellClientT: ShellClientTrait, EventHandlerT> EventHandler<ShellClientT> for EventHandlerT
where
    EventHandlerT: FnMut(Event, Option<&mut ShellClientT::Window>)
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
