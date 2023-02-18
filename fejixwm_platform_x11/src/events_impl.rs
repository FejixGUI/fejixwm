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
            .or_else(|_| Err(Error::PlatformApiFailed("cannot poll for next event")))
    }

}