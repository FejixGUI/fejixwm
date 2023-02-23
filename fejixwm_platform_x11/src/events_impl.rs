use crate::{
    types::*,
    core::events::*
};


/// Reduces function arguments boilerplate
struct WindowEventWrapper<'a, EventT> {
    pub window: &'a &'a mut X11Window,
    pub handler: EventHandlerRef<'a, X11ShellClient>,
    pub event: &'a EventT,
}


impl<'a, EventT> WindowEventWrapper<'a, EventT> {

    pub fn with<AnotherEventT>(self, event: &'a AnotherEventT) -> WindowEventWrapper<'a, AnotherEventT> {
        WindowEventWrapper { window: self.window, handler: self.handler, event }
    }

}


const SUCCESS: Result<EventResponse> = Ok(EventResponse::ContinueProcessing);



impl X11ShellClient {

    pub(crate) fn read_next_event(&self) -> Result<Option<xcb::Event>> {
        self.connection.poll_for_event()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot poll for next event")))
    }


    pub(crate) fn wait_for_event(&self) -> Result<xcb::Event> {
        self.connection.wait_for_event()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot wait for next event")))
    }


    fn find_window_by_handle<'a>(&self, windows: &'a [&'a mut Window], handle: X11WindowHandle)
        -> Option<&&'a mut Window>
    {
        windows.iter()
            .find(|window| window.handle == handle)
    }


    pub(crate) fn handle_event(&self, windows: &[&mut X11Window], event: xcb::Event, handler: EventHandlerRef<Self>)
        -> Result<EventResponse>
    {
        if let Some(window_handle) = self.get_event_window_handle(&event) {
            let window = self.find_window_by_handle(windows, window_handle)
                .ok_or(Error::IncompleteWindowList)?;

            return self.handle_window_event(WindowEventWrapper { window, event: &event, handler });
        } 

        return self.handle_global_event(&event, handler);
    }


    fn handle_window_event(&self, wrapper: WindowEventWrapper<xcb::Event>) -> Result<EventResponse> {
        match wrapper.event {
            xcb::Event::X(event) => self.handle_x_event(wrapper.with(event)),
            _ => todo!()
        }
    }


    fn handle_global_event(&self, event: &xcb::Event, handler: EventHandlerRef<Self>) -> Result<EventResponse> {
        // TODO
        SUCCESS
    }


    fn handle_x_event(&self, wrapper: WindowEventWrapper<xcb::x::Event>) -> Result<EventResponse> {
        match wrapper.event {
            xcb::x::Event::ClientMessage(event) => self.handle_client_message(wrapper.with(event)),

            // TODO handle more events
            _ => SUCCESS
        }
    }


    fn handle_client_message(&self, wrapper: WindowEventWrapper<xcb::x::ClientMessageEvent>) -> Result<EventResponse> {
        use xcb::Xid;

        let message_data = wrapper.event.data();
        
        if let xcb::x::ClientMessageData::Data32(data32) = wrapper.event.data() {

            let message_type = data32[0];

            if message_type == self.atoms.WM_DELETE_WINDOW.resource_id() {
                return self.handle_window_close(wrapper.with(&()));
            }
        }

        return SUCCESS;
    }


    fn handle_window_close(&self, wrapper: WindowEventWrapper<()>) -> Result<EventResponse> {
        Ok((wrapper.handler)(self, Some(wrapper.window), Event::Close))
    }


    fn get_event_window_handle(&self, event: &xcb::Event) -> Option<X11WindowHandle> {
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