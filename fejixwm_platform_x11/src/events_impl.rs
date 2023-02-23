use crate::{
    types::*,
    core::events::*
};


impl X11ShellClient {

    pub(crate) fn read_next_event(&self) -> Result<Option<xcb::Event>> {
        self.connection.poll_for_event()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot poll for next event")))
    }


    pub(crate) fn wait_for_event(&self) -> Result<xcb::Event> {
        self.connection.wait_for_event()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot wait for next event")))
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


    fn find_window_by_handle<'a>(&self, windows: &'a [&'a mut Window], handle: X11WindowHandle)
        -> Option<&'a &'a mut Window>
    {
        windows.iter()
            .find(|window| window.handle == handle)
    }


    pub(crate) fn handle_event(&self, windows: &[&mut X11Window], handler: EventHandlerRef<Self>)
        -> Result<EventResponse>
    {
        Ok(EventResponse::EndProcessing)
    }

}