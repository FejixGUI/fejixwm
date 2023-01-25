use crate::{
    types::*,
    core::events::*
};

impl WindowManager {

    pub(crate) fn next_event(&mut self) {
        let event = self.connection.wait_for_event()
            .map(|event| self.handle_event(event));
    }


    fn call_event_handler(&mut self, event: events::AnyEvent) {
        if self.event_handler.is_none() {
            panic!("no event handler was set");
        }

        let mut handler = self.event_handler.take().unwrap();
        handler(self, event);
        self.event_handler = Some(handler);
    }


    fn handle_event(&mut self, event: xcb::Event) {
        match event {
            xcb::Event::X(event) => self.handle_x_event(event),

            // TODO handle more events
            _ => {}
        }
    }


    fn handle_x_event(&mut self, event: xcb::x::Event) {
        match event {
            xcb::x::Event::ClientMessage(event) => {
                self.handle_x_client_message(event);
            }

            // TODO handle more X events
            _ => {}
        }
    }

    fn handle_x_client_message(&mut self, event: xcb::x::ClientMessageEvent) {
        use xcb::Xid;
        
        if let xcb::x::ClientMessageData::Data32([atom, ..]) = event.data() {
            if atom == self.atoms.WM_DELETE_WINDOW.resource_id() {
                self.handle_wm_delete_window(event);
            }

            // TODO other client messages
            // TODO smooth redraw
        }
    }


    fn handle_wm_delete_window(&mut self, event: xcb::x::ClientMessageEvent) {
        let event = AnyEvent::WindowEvent {
            window_id: self.get_window_id(event.window()).unwrap(),
            event: WindowEvent::Close
        };

        self.call_event_handler(event);
    }

}