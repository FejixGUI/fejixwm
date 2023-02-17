use crate::{
    errors::Result,
    events::EventHandler,
};


pub type WindowId = u32;


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
    /// This SHOULD be unique for every application.
    pub id: &'a str,
    
    /// Declares what protocols the client is going to use
    pub subsystems: &'a [ShellSubsystem],
}


#[derive(Clone)]
pub struct WindowInfo {
    pub size: PixelSize,

    /// Arbitrary number that is attached to the window and can be used by the program.
    /// This has no special meaning to FejixWM.
    /// After creation id can be accessed via [ShellClientTrait::get_window_id].
    pub id: WindowId,
}


pub trait ShellClientTrait : Sized {

    type Window;


    fn new(info: &ShellClientInfo) -> Result<Self>;

    /// Processes events and returns.
    /// The exact behavior of the function depends on the values returned from the event handler.
    fn process_windows<F: EventHandler<Self>>(&self, windows: &[&mut Self::Window], event_handler: F);

    /// Generates a special event that interrupts [ShellClientTrait::process_windows] while waiting for events.
    fn interrupt_waiting(&self);


    fn get_window_id(&self, window: &Self::Window) -> WindowId;


    fn is_subsystem_available(&self, subsystem: ShellSubsystem)
        -> bool;

    fn is_subsystem_enabled(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> bool;

    fn is_subsystem_forced(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> bool;

    /// Returns Ok(()) if the subsystem is already enabled
    fn enable_subsystem(&self, window: &mut Self::Window, subsystem: ShellSubsystem)
        -> Result<()>;

    /// Returns Ok(()) if the subsystem is already disabled
    fn disable_subsystem(&self, window: &mut Self::Window, subsystem: ShellSubsystem)
        -> Result<()>;

    /// Returns `Err` if toggling a subsystem would result in an error.
    /// 
    /// Used in `enable/disable_subsystem` AFTER checking whether the subsystem is already enabled/disabled
    fn check_subsystem_toggleable(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> Result<()>
    {
        use crate::errors::Error;

        if !self.is_subsystem_available(subsystem) {
            return Err(Error::SubsystemNotAvailable);
        }

        if self.is_subsystem_forced(window, subsystem) {
            return Err(Error::SubsystemForced);
        }

        Ok(())
    }

}


pub trait CanvasTrait : Sized {

    type ShellClient : ShellClientTrait;

    /// Must be equal to `<Self::ShellClient as ShellClientTrait>::Window`
    type Window;

    type CanvasInfo;

    fn new(client: &Self::ShellClient, window_info: &WindowInfo, canvas_info: &Self::CanvasInfo)
        -> Result<(Self::Window, Self)>;

    fn drop(self, client: &Self::ShellClient, window: Self::Window)
        -> Result<()>;

}


pub trait ShellSubsystemManager<_T> {
    
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


impl ShellSubsystem {
    pub fn list() -> &'static [Self] {
        &[
            Self::KeyboardInput,
            Self::MouseInput,
            Self::SysRedraw,
            Self::TextInput,
        ]
    }
}