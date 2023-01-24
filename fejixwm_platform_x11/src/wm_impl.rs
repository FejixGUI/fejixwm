use crate::types::*;


impl WindowManagerTrait for WindowManager {
    fn new(info: &WindowManagerInfo) -> Result<Self> {
        Self::new(info)
    }

    fn new_window(&mut self, info: &WindowInfo) -> Result<()> {
        self.create_window(info)
    }

    fn drop_window(&mut self, wid: WindowId) -> Result<()> {
        self.destroy_window(wid)
    }

    fn run(&mut self, event_handler: impl EventHandler) {
        self.run_event_loop(event_handler);
    }
}


impl Drop for WindowManager {
    fn drop(&mut self) {
        self.destroy().unwrap();
    }
}



impl WindowManager {
    pub fn new(info: &WindowManagerInfo) -> Result<Self> {
        let (connection, default_screen_number) = Self::connect()?;
        let input_method = Self::create_input_method(&connection)?;
        let atoms = XAtoms::intern_all(&connection)
            .or_else(|_| Err(Error::PlatformApiFailed("cannot get X atoms")))?;

        Ok(Self {
            name: info.name.clone(),

            connection,
            default_screen_number,
            atoms,
            input_method,

            window_handles: WindowStorage::new(),
            window_state_cache: WindowStorage::new(),
            smooth_redraw_drivers: WindowStorage::new(),
            text_input_drivers: WindowStorage::new(),
        })
    }


    fn destroy(&mut self) -> Result<()> {
        self.destroy_windows()?;
        self.detroy_input_method();
        Ok(())
    }


    fn register_window(&mut self, wid: WindowId, xwindow_id: xcb::x::Window) {
        self.window_handles.insert(wid, xwindow_id);
    }


    fn unregister_window(&mut self, wid: &WindowId) {
        self.window_handles.remove(&wid);
    }


    pub(crate) fn get_window_handle(&self, wid: &WindowId) -> Result<xcb::x::Window> {
        let xwindow = self.window_handles.get(&wid)
            .ok_or_else(|| Error::InvalidArgument)?;

        Ok(xwindow.clone())
    }


    pub(crate) fn get_default_screen(&self) -> &xcb::x::Screen {
        self.connection.get_setup().roots().nth(self.default_screen_number as usize).unwrap()
    }


    fn connect() -> Result<(xcb::Connection, i32)> {
        xcb::Connection::connect_with_xlib_display()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot connect to Xorg")))
    }


    fn create_input_method(connection: &xcb::Connection) -> Result<xlib::XIM> {
        let im = unsafe {
            xlib::XOpenIM(connection.get_raw_dpy(), null_mut(), null_mut(), null_mut())
        };
        
        if im.is_null() {
            return Err(Error::PlatformApiFailed("cannot create input method"));
        }

        return Ok(im)
    }


    fn detroy_input_method(&mut self) {
        unsafe {
            xlib::XCloseIM(self.input_method);
        }
    }


    pub(crate) fn create_window(&mut self, info: &WindowInfo) -> Result<()> {
        let visual_info: WindowVisualInfo = self.create_visual_info(info)?;
        let wid = info.id;

        let xwindow = self.create_xwindow(info, &visual_info)?;
        self.register_window(wid, xwindow);
        self.set_xwindow_protocols(wid, info)?;
        self.init_window_drivers(wid, info.flags)?;
        self.create_window_canvas(wid, info)?;

        Ok(())
    }


    pub(crate) fn destroy_window(&mut self, wid: WindowId) -> Result<()> {
        self.destroy_window_drivers(wid)?;
        self.destroy_xwindow(wid)?;
        self.unregister_window(&wid);
        Ok(())
    }


    fn create_xwindow(&self, info: &WindowInfo, visual_info: &WindowVisualInfo) -> Result<xcb::x::Window> {
        let xwindow_id = self.connection.generate_id();

        let default_screen = self.get_default_screen();

        self.connection.send_and_check_request(&xcb::x::CreateWindow {
            wid: xwindow_id,
            parent: default_screen.root(),
            class: xcb::x::WindowClass::InputOutput,
            
            x: 0,
            y: 0,
            width: info.size.width as u16,
            height: info.size.height as u16,
            border_width: 0,
            
            // TODO Is screen depth important?
            depth: xcb::x::COPY_FROM_PARENT as u8,
            visual: visual_info.visualid,
            value_list: &[
                xcb::x::Cw::BackPixel(default_screen.black_pixel()),
                xcb::x::Cw::EventMask(xcb::x::EventMask::all()),
                xcb::x::Cw::Colormap(visual_info.colormap)
            ]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot create window")))?;

        Ok(xwindow_id)
    }


    fn destroy_xwindow(&mut self, wid: WindowId) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::DestroyWindow {
            window: self.window_handles[&wid]
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy window")))
    }


    fn destroy_windows(&mut self) -> Result<()> {
        let window_ids = self.window_handles.keys().copied().collect::<Vec<u32>>();
        for wid in window_ids {
            self.drop_window(wid)?;
        }

        Ok(())
    }


    fn create_xwindow_protocols_list(&self, window_info: &core::WindowInfo) -> Vec<xcb::x::Atom> {
        let mut protocols = vec![
            self.atoms.WM_DELETE_WINDOW
        ];

        if window_info.flags.contains(core::WindowFlags::SMOOTH_REDRAW) {
            protocols.push(self.atoms._NET_WM_SYNC_REQUEST);
        }

        protocols
    }

    
    fn set_xwindow_protocols(&self, wid: WindowId, window_info: &core::WindowInfo) -> Result<()> {
        let xwindow = self.get_window_handle(&wid)?;
        let protocols = self.create_xwindow_protocols_list(window_info);

        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: xwindow,
            property: self.atoms.WM_PROTOCOLS,
            r#type: xcb::x::ATOM_ATOM,
            data: protocols.as_slice()
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set WM protocols")))?;

        Ok(())
    }


    fn init_window_drivers(&mut self, wid: WindowId, flags: WindowFlags) -> Result<()> {
        if flags.contains(WindowFlags::SMOOTH_REDRAW) {
            (self as &mut dyn WmSmoothRedrawDriver).new_driver(wid)?;
        }

        if flags.contains(WindowFlags::TEXT_INPUT) {
            (self as &mut dyn WmTextInputDriver).new_driver(wid)?;
        }

        Ok(())
    }


    fn destroy_window_drivers(&mut self, wid: WindowId) -> Result<()> {
        (self as &mut dyn WmSmoothRedrawDriver).drop_driver(wid)?;
        (self as &mut dyn WmTextInputDriver).drop_driver(wid)?;
        Ok(())
    }

    fn get_default_visual_info(&self) -> WindowVisualInfo {
        let screen = self.get_default_screen();
        
        WindowVisualInfo {
            visualid: screen.root_visual(),
            colormap: screen.default_colormap(),
        }
    }


    fn create_visual_info(&mut self, window_info: &WindowInfo) -> Result<WindowVisualInfo> {
        match window_info.canvas_info {
            CanvasInfo::None => Ok(self.get_default_visual_info()),

            // TODO implement graphics APIs
            _ => Err(Error::UnsupportedFeature),
        }
    }


    fn create_window_canvas(&mut self, wid: WindowId, info: &WindowInfo) -> Result<()> {
        match info.canvas_info {
            CanvasInfo::None => Ok(()),

            // TODO implement graphics APIs
            _ => Err(Error::UnsupportedFeature)
        }
    }


    fn run_event_loop(&mut self, mut event_handler: impl EventHandler) {

        loop {

            let event = self.connection.wait_for_event();

            if event.is_err() {
                break;
            }

            match event.unwrap() {
                xcb::Event::X(event) => self.handle_x_event(&mut event_handler, event),

                _ => {}
            }

        }

    }


    fn handle_x_event(&mut self, event_handler: &impl EventHandler, event: xcb::x::Event) {
        match event {
            xcb::x::Event::ClientMessage(event) => {

            }

            _ => {}
        }
    }

}

