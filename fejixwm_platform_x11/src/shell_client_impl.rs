use crate::types::*;



impl X11ShellClient {
    pub fn new(info: &ShellClientInfo) -> Result<Self> {
        let (connection, default_screen_number) = Self::connect()?;
        let xdisplay = connection.get_raw_dpy();
        let atoms = Self::get_atoms(&connection)?;
        let class_name = Self::translate_class_name(info.id)?;

        let mut myself = Self {
            connection,
            xdisplay,
            default_screen_number,
            atoms,
            class_name,

            text_input_subsystem: None,
        };

        myself.init_global_subsystems(info.subsystems)?;

        Ok(myself)
    }


    pub(crate) fn get_default_screen(&self) -> &xcb::x::Screen {
        self.connection.get_setup().roots().nth(self.default_screen_number as usize).unwrap()
    }


    pub(crate) fn get_default_visual_info(&self) -> X11WindowVisualInfo {
        let screen = self.get_default_screen();
        
        X11WindowVisualInfo {
            visualid: screen.root_visual(),
            colormap: screen.default_colormap(),
            color_depth: screen.root_depth(),
        }
    }


    pub(crate) fn get_default_wm_protocols(&self) -> Vec<xcb::x::Atom> {
        vec![
            self.atoms.WM_DELETE_WINDOW,
            self.atoms._NET_WM_PING,
        ]
    }


    fn connect() -> Result<(xcb::Connection, i32)> {
        xcb::Connection::connect_with_xlib_display()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot connect to Xorg")))
    }


    fn get_atoms(connection: &xcb::Connection) -> Result<X11Atoms> {
        X11Atoms::intern_all(&connection)
            .or_else(|_| Err(Error::PlatformApiFailed("cannot get X atoms")))
    }


    fn init_global_subsystems(&mut self, subsystem_list: &[ShellSubsystem]) -> Result<()> {
        if subsystem_list.contains(&ShellSubsystem::TextInput) {
            self.text_input_subsystem = Some(X11GlobalTextInputSubsystem::new(&self)?);
        }

        Ok(())
    }


    fn destroy_global_subsystems(&mut self) -> Result<()> {
        if let Some(subsystem) = self.text_input_subsystem.take() {
            subsystem.destroy();
        }

        Ok(())
    }


    fn translate_class_name(class_name: &str) -> Result<String> {
        if !class_name.is_ascii() {
            return Err(Error::InvalidArgument);
        }

        let mut result = String::new();
        result.push_str(class_name);
        result.push('\0');
        result.push_str(class_name);
        result.push('\0');
        Ok(result)
    }


    fn create_window_handle(&self, info: &WindowInfo, visual_info: &X11WindowVisualInfo) -> Result<X11WindowHandle> {
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
            
            depth: visual_info.color_depth,
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


    fn destroy_window_handle(&self, window_handle: X11WindowHandle) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::DestroyWindow {
            window: window_handle,
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy window")))
    }

    
    fn set_window_protocols(&self, window_handle: X11WindowHandle, protocols: &[xcb::x::Atom]) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: window_handle,
            property: self.atoms.WM_PROTOCOLS,
            r#type: xcb::x::ATOM_ATOM,
            data: protocols
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set WM protocols")))?;

        Ok(())
    }


    fn set_window_class(&self, window_handle: X11WindowHandle) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: window_handle,
            property: self.atoms.WM_CLASS,
            r#type: xcb::x::ATOM_STRING,
            data: self.class_name.as_bytes()
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set WM class name")))?;
        
        Ok(())
    }


    fn get_window_initial_state(&self, window_handle: X11WindowHandle, info: &WindowInfo) -> X11WindowState {
        X11WindowState {
            size: info.size.clone()
        }
    }


    fn get_window_protocols_list(&self, window: &X11Window) -> Vec<xcb::x::Atom> {
        let mut protocols = self.get_default_wm_protocols();

        if window.sys_redraw.is_some() {
            protocols.push(self.atoms._NET_WM_SYNC_REQUEST);
        }

        protocols
    }


    pub(crate) fn new_window(&self, info: &WindowInfo, visual_info: &X11WindowVisualInfo) -> Result<X11Window> {
        let window_handle = self.create_window_handle(info, visual_info)?;
        self.set_window_class(window_handle)?;
        self.set_window_protocols(window_handle, &self.get_default_wm_protocols())?;
        let state = self.get_window_initial_state(window_handle, info);

        Ok(X11Window {
            id: info.id,
            handle: window_handle,
            state,
            text_input: None,
            sys_redraw: None,
        })
    }


    pub(crate) fn drop_window(&self, mut window: X11Window) -> Result<()> {
        self.destroy_window_subsystems(&mut window)?;
        self.destroy_window_handle(window.handle)?;
        Ok(())
    }


    fn destroy_window_subsystems(&self, window: &mut X11Window) -> Result<()> {
        for subsystem in ShellSubsystem::list() {
            if !self.is_subsystem_forced(window, subsystem.clone()) {
                self.disable_subsystem(window, subsystem.clone())?;
            }
        }   

        Ok(())
    }


}



impl ShellClientTrait for X11ShellClient {

    type Window = X11Window;


    fn new(info: &ShellClientInfo) -> Result<Self> {
        X11ShellClient::new(info)
    }


    fn get_window_id(&self, window: &Self::Window) -> WindowId {
        window.id
    }

    fn process_windows<F: events::EventHandler<Self>>(&self, windows: &[&mut Self::Window], event_handler: F) {
        todo!()
    }

    fn interrupt_waiting(&self) {
        todo!()
    }

    fn is_subsystem_available(&self, subsystem: ShellSubsystem) -> bool {
        match subsystem {
            ShellSubsystem::TextInput => { self.text_input_subsystem.is_some() },
            _ => true
        }
    }

    fn is_subsystem_enabled(&self, window: &Self::Window, subsystem: ShellSubsystem) -> bool {
        match subsystem {
            ShellSubsystem::TextInput => { window.text_input.is_some() },
            ShellSubsystem::SysRedraw => { window.sys_redraw.is_some() },
            _ => true,
        }
    }

    fn is_subsystem_forced(&self, window: &Self::Window, subsystem: ShellSubsystem) -> bool {
        match subsystem {
            ShellSubsystem::KeyboardInput
            | ShellSubsystem::MouseInput => true,
            _ => false,
        }
    }

    fn enable_subsystem(&self, window: &mut Self::Window, subsystem: ShellSubsystem) -> Result<()> {
        if self.is_subsystem_enabled(window, subsystem) {
            return Ok(())
        }

        self.check_subsystem_toggleable(window, subsystem)?;

        match subsystem {
            ShellSubsystem::SysRedraw => {
                window.sys_redraw = Some(X11SysRedrawSubsystem::new(self, window.handle)?);
            }

            ShellSubsystem::TextInput => {
                window.text_input = Some(X11TextInputSubsystem::new(self, window.handle)?);
            }

            _ => {}
        }

        Ok(())
    }

    fn disable_subsystem(&self, window: &mut Self::Window, subsystem: ShellSubsystem) -> Result<()> {
        if !self.is_subsystem_enabled(window, subsystem) {
            return Ok(())
        }
        
        self.check_subsystem_toggleable(window, subsystem)?;

        match subsystem {
            ShellSubsystem::SysRedraw => {
                let subsystem = window.sys_redraw.take().unwrap();
                subsystem.destroy(self)?;
            }

            ShellSubsystem::TextInput => {
                let subsystem = window.text_input.take().unwrap();
                subsystem.destroy();
            }

            _ => {}
        }

        Ok(())
    }

}