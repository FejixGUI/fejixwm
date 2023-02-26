use crate::{
    types::*,
    core::events::*
};


/// Reduces function arguments boilerplate
// struct WindowEventWrapper<'a, EventT> {
//     pub window: &'a mut Window,
//     pub handler: &'a mut impl EventHandler<ShellClient>,
//     pub event: &'a EventT,
// }


// impl<'a, EventT> WindowEventWrapper<'a, EventT> {

//     pub fn with<AnotherEventT>(self, event: &'a AnotherEventT) -> WindowEventWrapper<'a, AnotherEventT> {
//         WindowEventWrapper { window: self.window, handler: self.handler, event }
//     }

// }




impl ShellClient {

    pub(crate) fn handle_message(
        &self, window: Option<&mut Window>, event: &ShellMessage, handler: impl EventHandler<Self>
    )
        -> Result<()>
    {
        Ok(())
    }

    /* fn handle_window_event(&self, wrapper: WindowEventWrapper<xcb::Event>) -> Result<EventListeningBehavior> {
        match wrapper.event {
            xcb::Event::X(event) => self.handle_x_event(wrapper.with(event)),
            _ => todo!()
        }
    }


    fn handle_global_event(&self, event: &xcb::Event, handler: EventHandlerRef<Self>) -> Result<EventListeningBehavior> {
        // TODO
        SUCCESS
    }


    fn handle_x_event(&self, wrapper: WindowEventWrapper<xcb::x::Event>) -> Result<EventListeningBehavior> {
        match wrapper.event {
            xcb::x::Event::ClientMessage(event) => self.handle_client_message(wrapper.with(event)),

            // TODO handle more events
            _ => SUCCESS
        }
    }


    fn handle_client_message(&self, wrapper: WindowEventWrapper<xcb::x::ClientMessageEvent>) -> Result<EventListeningBehavior> {
        use xcb::Xid;

        let message_data = wrapper.event.data();
        
        if let xcb::x::ClientMessageData::Data32(data32) = wrapper.event.data() {

            let message_type = data32[0];

            if message_type == self.atoms.WM_DELETE_WINDOW.resource_id() {
                return self.handle_window_close(wrapper.with(&()));
            } else if message_type == self.atoms._NET_WM_PING.resource_id() {
                self.handle_ping(wrapper)?;
                return SUCCESS;
            }
        }

        return SUCCESS;
    }


    fn handle_window_close(&self, wrapper: WindowEventWrapper<()>) -> Result<EventListeningBehavior> {
        Ok((wrapper.handler)(self, Some(wrapper.window), Event::Close))
    }


    fn handle_ping(&self, wrapper: WindowEventWrapper<xcb::x::ClientMessageEvent>) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(self.get_default_window()),
            event_mask: xcb::x::EventMask::all(),
            event: wrapper.event
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot respond to system ping")))?;
        
        Ok(())
    } */


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
                    // Got GraphicsExpose event (allowed by XSetGraphicsExposures)
                    GraphicsExposure(event) => None,
                    GravityNotify(event) => Some(event.window()),
                    KeyPress(event) => Some(event.event()),
                    KeyRelease(event) => Some(event.event()),
                    // Global keymap event
                    KeymapNotify(event) => None,
                    LeaveNotify(event) => Some(event.event()),
                    MapNotify(event) => Some(event.window()),
                    MapRequest(event) => Some(event.window()),
                    // Global keymap event
                    MappingNotify(event) => None,
                    MotionNotify(event) => Some(event.event()),
                    // Got NoExpose event (allowed by XSetGraphicsExposures)
                    NoExposure(event) => None,
                    PropertyNotify(event) => Some(event.window()),
                    ReparentNotify(event) => Some(event.window()),
                    ResizeRequest(event) => Some(event.window()),
                    // Clipboard events
                    SelectionClear(event) => None,
                    SelectionNotify(event) => None,
                    SelectionRequest(event) => None,
                    
                    UnmapNotify(event) => Some(event.window()),
                    VisibilityNotify(event) => Some(event.window()),
                }
            }

            _ => todo!()
        }
    }

}