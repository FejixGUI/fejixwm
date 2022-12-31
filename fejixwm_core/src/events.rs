use crate::core::*;


pub enum AnyEvent<'a, WmApiT : WmApi> {
    WindowEvent {
        window: &'a WmApiT::Window,
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