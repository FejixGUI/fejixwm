use crate::types::*;



impl ShellClient {
    pub fn new(info: &ShellClientInfo) -> Result<Self> {
        let (connection, default_screen_number) = Self::connect()?;
        let atoms = Self::get_atoms(&connection)?;

        let myself = Self {
            connection,
            xdisplay: connection.get_raw_dpy(),
            default_screen_number,
            atoms,
            class_name: info.id.to_string(),

            text_input_subsystem: None,
        };

        myself.enable_global_subsystems(info.subsystems)?;

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


    fn connect() -> Result<(xcb::Connection, i32)> {
        xcb::Connection::connect_with_xlib_display()
            .or_else(|_| Err(Error::PlatformApiFailed("cannot connect to Xorg")))
    }


    fn get_atoms(connection: &xcb::Connection) -> Result<X11Atoms> {
        X11Atoms::intern_all(&connection)
            .or_else(|_| Err(Error::PlatformApiFailed("cannot get X atoms")))
    }


    fn enable_global_subsystems(&mut self, subsystem_list: &[ShellSubsystem]) -> Result<()> {
        if subsystem_list.contains(&ShellSubsystem::TextInput) {
            self.text_input_subsystem = Some(X11GlobalTextInputSubsystem::new(&self)?);
        }

        Ok(())
    }


    fn create_xwindow(&self, info: &WindowInfo, visual_info: &X11WindowVisualInfo) -> Result<xcb::x::Window> {
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


    fn destroy_xwindow(&mut self, window_handle: X11WindowHandle) -> Result<()> {
        self.connection.send_and_check_request(&xcb::x::DestroyWindow {
            window: window_handle,
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot destroy window")))
    }

    fn create_xwindow_protocols_list(&self, window_info: &WindowInfo) -> Vec<xcb::x::Atom> {
        let mut protocols = vec![
            self.atoms.WM_DELETE_WINDOW
        ];

        if true /* TODO Window protocol selection */ {
            protocols.push(self.atoms._NET_WM_SYNC_REQUEST);
        }

        protocols
    }

    
    fn set_xwindow_protocols(&self, window_handle: X11WindowHandle, window_info: &WindowInfo) -> Result<()> {
        let protocols = self.create_xwindow_protocols_list(window_info);

        self.connection.send_and_check_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: window_handle,
            property: self.atoms.WM_PROTOCOLS,
            r#type: xcb::x::ATOM_ATOM,
            data: protocols.as_slice()
        })
        .or_else(|_| Err(Error::PlatformApiFailed("cannot set WM protocols")))?;

        Ok(())
    }


}