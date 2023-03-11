use crate::{
    errors::*,
    events::{Event, EventHandler, MessageCallback, ShellMessageTrait},
};


use std::{
    any::Any
};


/// An identifier of a FejixWM window created by the client program.
/// 
/// Often corresponds to the window handle, but does not have the same meaning as the handle.
/// For example, shell messages frequently contain window handles that do not point to the windows created by the
/// FejixWM user. Such handles may point to the shell's root window, to the internal fake window created by FejixWM
/// or even to NULL. We say that such messages contain window handles but do not contain window identifiers.
pub type WindowId = usize;


#[derive(Clone)]
pub struct PixelSize {
    pub width: u32,
    pub height: u32
}


#[derive(Clone, Copy, PartialEq)]
pub enum ShellSubsystem {
    MouseInput,
    KeyboardInput,
    TextInput,
    SysRedraw,
}


#[derive(Clone)]
pub struct ShellClientInfo<'a> {
    /// ASCII string that represents the identifier of the program, often called "window class" by numerous platforms.
    /// This SHOULD be unique for each application.
    pub id: &'a str,
    
    /// Declares what protocols of the shell the client is going to use
    pub subsystems: &'a [ShellSubsystem],
}


// TODO Window class (default/tooltip/splash/menu/etc.), flags, parent, etc.
#[derive(Clone)]
pub struct WindowInfo {
    pub size: PixelSize,
}


pub trait WindowTrait : Sized {

    fn get_id(&self) -> WindowId;

    /// Returns the cached size. The cached size is updated by [ShellClientTrait::process_message].
    fn get_size(&self) -> PixelSize;

}


pub trait ShellClientTrait : Sized {

    type Window : WindowTrait;

    type ShellMessage : ShellMessageTrait;


    fn new(info: &ShellClientInfo)
        -> Result<Self>;


    /// Runs a loop that receives system messages from the shell.
    /// 
    /// The message callback should find a window by the identifier possibly contained in the message and process
    /// the message with [ShellClientTrait::process_message]
    fn listen_to_messages(&self, callback: impl MessageCallback<Self>)
        -> Result<()>;

    /// Translates the system message to zero or more FejixWM events, handles them and modifies the window state.
    /// 
    /// The `window` should be `None` if the event is global.
    fn process_message(
        &self, message: &Self::ShellMessage, window: Option<&mut Self::Window>, handler: impl EventHandler<Self>
    ) -> Result<()>;


    /// Returns `Err` if processing the message with the provided optional window makes no sense.
    fn assert_can_process_message(&self, message: &Self::ShellMessage, window: &Option<&mut Self::Window>)
        -> Result<()>
    {
        if (message.is_global() && window.is_none()) || (!message.is_global() && window.is_some()) {
            Ok(())
        } else {
            Err(Error::InvalidArgument)
        }
    }


    /// Generates an artificial message and wakes up the thread listening to messages.
    /// 
    /// The generated message will be translated to a [crate::events::UserEvent].
    fn post_message(&self, user_data: Option<Box<dyn Any>>)
        -> Result<()>;


    /// Returns true if the subystem is globally available
    fn is_subsystem_available(&self, subsystem: ShellSubsystem)
        -> bool;

    /// Returns true if the subystem is enabled for the window
    fn is_subsystem_enabled(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> bool;
        
    /// Returns true if the subystem's state is forced and cannot be changed
    fn is_subsystem_forced(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> bool;

    /// If the subsystem is already enabled, returns Ok(()) 
    fn enable_subsystem(&self, window: &mut Self::Window, subsystem: ShellSubsystem)
        -> Result<()>;

    /// If the subsystem is already disabled, returns Ok(()) 
    fn disable_subsystem(&self, window: &mut Self::Window, subsystem: ShellSubsystem)
        -> Result<()>;

    /// Returns `Err` if toggling the subsystem results in error.
    /// 
    /// Returns `Ok(false)` if the state is already set and there is nothing to do.
    /// 
    /// Returns `Ok(true)` otherwise.
    /// 
    /// Used by [ShellClientTrait::enable_subsystem] and [ShellClientTrait::disable_subsystem].
    fn can_set_subsystem_state(&self, window: &Self::Window, subsystem: ShellSubsystem, state_enabled: bool)
        -> Result<bool>
    {
        if self.is_subsystem_enabled(window, subsystem) == state_enabled {
            return Ok(false)
        }

        if !self.is_subsystem_available(subsystem) {
            return Err(Error::SubsystemNotAvailable);
        }

        if self.is_subsystem_forced(window, subsystem) {
            return Err(Error::SubsystemForced);
        }

        Ok(true)
    }


    /// Asks the shell for the current window size.
    fn get_window_size(&self, window: &Self::Window)
        -> Result<PixelSize>;


}



pub trait CanvasTrait : Sized {

    type ShellClient : ShellClientTrait;

    /// Makes function types simpler.
    /// 
    /// Must be equal to `<Self::ShellClient as ShellClientTrait>::Window`
    type Window;

    type CanvasInfo;

    fn new(client: &Self::ShellClient, window_info: &WindowInfo, canvas_info: &Self::CanvasInfo)
        -> Result<(Self::Window, Self)>;

    fn drop(self, client: &Self::ShellClient, window: Self::Window)
        -> Result<()>;

}



impl PixelSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl std::fmt::Display for PixelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.width, self.height)
    }
}


impl PartialEq for PixelSize {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}


impl ShellSubsystem {
    pub fn all() -> &'static [Self] {
        &[
            Self::KeyboardInput,
            Self::MouseInput,
            Self::SysRedraw,
            Self::TextInput,
        ]
    }
}