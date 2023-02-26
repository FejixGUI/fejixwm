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
    /// Sent by [ShellClientTrait::trigger_event]
    Trigger,

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


#[derive(Default)]
pub struct EventListeningSettings {
    pub behavior: EventListeningBehavior
}


/// No shell event is sent when the current behavior is [EventListeningBehavior::GetNextEvent] but there are no events.
pub trait MessageCallback<ShellClientT: ShellClientTrait>
    : FnMut(Option<&ShellClientT::ShellMessage>, &mut EventListeningSettings)
{}

/// No window is passed when the event is global
pub trait EventHandler<ShellClientT: ShellClientTrait>
    : FnMut(Event, Option<&mut ShellClientT::Window>)
{}

// Make all closures that look like message callbacks actual message callbacks
impl<ShellClientT: ShellClientTrait, EventCallbackT> MessageCallback<ShellClientT> for EventCallbackT
where
    EventCallbackT: FnMut(Option<&ShellClientT::ShellMessage>, &mut EventListeningSettings)
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

impl Default for EventListeningBehavior {
    fn default() -> Self {
        Self::GetNextEvent
    }
}