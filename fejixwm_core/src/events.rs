use crate::core::*;

use std::{
    any::Any,
    ops::FnMut,
};


pub enum Event {
    /// Sent after all available events have been processed in order to query for an event outcome
    NoMoreEvents,

    /// Sent by [ShellClientTrait::interrupt_waiting]
    WaitingInterrupted,

    Close,
    Resize { new_size: PixelSize },
    UserEvent { data: Box<dyn Any> }
}


pub enum EventOutcome {
    /// Makes [ShellClientTrait::process_windows] continue processing events or generate [Event::NoMoreEvents] there
    /// are no events left.
    ContinueProcessing,

    /// Makes [ShellClientTrait::process_windows] immediately return.
    EndProcessing,

    /// Makes [ShellClientTrait::process_windows] block its thread until any new events are received.
    WaitForEvents,
}


pub trait EventHandler<ShellClientT: ShellClientTrait>
    : 'static + FnMut(&ShellClientT, &mut ShellClientT::Window, Event) -> EventOutcome
{}

// Make all closures that look like event handlers actual event handlers
impl<ShellClientT: ShellClientTrait, EventHandlerT> EventHandler<ShellClientT> for EventHandlerT
where
    EventHandlerT: 'static + FnMut(&ShellClientT, &mut ShellClientT::Window, Event) -> EventOutcome
{}


impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMoreEvents => write!(f, "no more events left to process"),
            Self::WaitingInterrupted => write!(f, "wake up"),
            Self::Close => write!(f, "closed"),
            Self::Resize { new_size } => write!(f, "resized to {new_size}"),
            Self::UserEvent { .. } => write!(f, "user-defined event")
        }
    }
}
