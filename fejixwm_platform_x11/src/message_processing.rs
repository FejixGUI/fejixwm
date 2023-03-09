use crate::{
    types::*,
    core::events::*
};


/// Reduces function arguments boilerplate
struct EventWrapper<'a, EventT> {
    pub window: Option<&'a mut Window>,
    pub handler: &'a mut dyn EventHandler<ShellClient>,
    pub event: &'a EventT,
}


impl<'a, EventT> EventWrapper<'a, EventT> {

    pub fn with<AnotherEventT>(self, event: &'a AnotherEventT) -> EventWrapper<'a, AnotherEventT> {
        EventWrapper { window: self.window, handler: self.handler, event }
    }

}




impl ShellClient {

    pub(crate) fn handle_message(
        &self, message: &ShellMessage, window: Option<&mut Window>, mut handler: impl EventHandler<Self>
    )
        -> Result<()>
    {
        let wrapper = EventWrapper { event: &message.event, window, handler: &mut handler };

        if wrapper.window.is_some() {
            self.handle_window_event(wrapper)
        } else {
            self.handle_global_event(wrapper)
        }
    }

    fn handle_window_event(&self, wrapper: EventWrapper<xcb::Event>) -> Result<()> {
        match wrapper.event {
            xcb::Event::X(event) => self.handle_x_event(wrapper.with(event)),
            _ => todo!()
        }
    }


    fn handle_global_event(&self, wrapper: EventWrapper<xcb::Event>) -> Result<()> {
        // TODO global events
        Ok(())
    }


    fn handle_x_event(&self, wrapper: EventWrapper<xcb::x::Event>) -> Result<()> {
        match wrapper.event {
            xcb::x::Event::ClientMessage(event) =>
                self.handle_client_message(wrapper.with(event)),

            // TODO handle more events
            _ => Ok(())
        }
    }


    fn handle_client_message(&self, wrapper: EventWrapper<xcb::x::ClientMessageEvent>) -> Result<()> {
        use xcb::Xid;

        let message_data = wrapper.event.data();
        
        if let xcb::x::ClientMessageData::Data32(data32) = wrapper.event.data() {
            let message_type = data32[0];

            if message_type == self.atoms.WM_DELETE_WINDOW.resource_id() {
                self.handle_window_close(wrapper.with(&()))?;
            } else if message_type == self.atoms._NET_WM_PING.resource_id() {
                self.handle_ping(wrapper)?;
            }
        }

        // TODO handle user event

        return Ok(());
    }


    fn handle_window_close(&self, wrapper: EventWrapper<()>) -> Result<()> {
        (wrapper.handler)(Event::WindowEvent(WindowEvent::Close), wrapper.window);
        Ok(())
    }


    fn handle_ping(&self, wrapper: EventWrapper<xcb::x::ClientMessageEvent>) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(self.get_default_window()),
            event_mask: xcb::x::EventMask::all(),
            event: wrapper.event
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot respond to system ping")))?;
        
        Ok(())
    }


    pub(crate) fn get_event_window_handle(&self, event: &xcb::Event) -> Option<X11WindowHandle> {
        match event {
            xcb::Event::X(event) => {
                use xcb::x::Event::*;
                match event {
                    ButtonPress(event) => Some(event.event()),
                    ButtonRelease(event) => Some(event.event()),
                    CirculateNotify(event) => Some(event.window()),
                    CirculateRequest(event) => Some(event.window()),
                    ClientMessage(event) => Some(event.window()),
                    ColormapNotify(event) => Some(event.window()),
                    ConfigureNotify(event) => Some(event.window()),
                    ConfigureRequest(event) => Some(event.window()),
                    CreateNotify(event) => Some(event.window()),
                    DestroyNotify(event) => Some(event.window()),
                    EnterNotify(event) => Some(event.event()),
                    Expose(event) => Some(event.window()),
                    FocusIn(event) => Some(event.event()),
                    FocusOut(event) => Some(event.event()),
                    GravityNotify(event) => Some(event.window()),
                    KeyPress(event) => Some(event.event()),
                    KeyRelease(event) => Some(event.event()),
                    LeaveNotify(event) => Some(event.event()),
                    MapNotify(event) => Some(event.window()),
                    MapRequest(event) => Some(event.window()),
                    MotionNotify(event) => Some(event.event()),
                    PropertyNotify(event) => Some(event.window()),
                    ReparentNotify(event) => Some(event.window()),
                    ResizeRequest(event) => Some(event.window()),
                    UnmapNotify(event) => Some(event.window()),
                    VisibilityNotify(event) => Some(event.window()),

                    // Got GraphicsExpose event (allowed by XSetGraphicsExposures)
                    GraphicsExposure(event) => None,
                    // Got NoExpose event (allowed by XSetGraphicsExposures)
                    NoExposure(event) => None,
                    // Global keymap event
                    KeymapNotify(event) => None,
                    // Global keymap event
                    MappingNotify(event) => None,
                    // Clipboard events
                    SelectionClear(event) => None,
                    SelectionNotify(event) => None,
                    SelectionRequest(event) => None,
                }
            }

            _ => todo!()
        }
    }

}