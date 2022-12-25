use crate::interface::core::*;


pub enum AnyEvent<'a> {
    WindowEvent {
        window: &'a Window,
        event: WindowEvent,
    },

    GlobalEvent {
        event: GlobalEvent
    }
}


pub enum WindowEvent {
    Close,
    Redraw,
}


pub enum GlobalEvent {

}