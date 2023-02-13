use crate::{
    errors::Result,
    events::EventHandler,
};


#[derive(Clone)]
pub struct PixelSize {
    pub width: u32,
    pub height: u32
}


#[derive(Clone, PartialEq)]
pub enum ShellSubsystem {
    MouseInput,
    KeyboardInput,
    TextInput,
    SysRedraw,
}


#[derive(Clone)]
pub struct ShellClientInfo<'a> {
    pub id: &'a str,
    pub subsystems: &'a [ShellSubsystem],
}


#[derive(Clone)]
pub struct WindowInfo {
    pub size: PixelSize,
}


pub trait ShellClientTrait : Sized {

    type Window;


    fn new(info: &ShellClientInfo) -> Result<Self>;

    fn new_window_with_canvas<CanvasT>(&self, window_info: &WindowInfo, canvas_info: &CanvasT::CanvasInfo)
        -> Result<(Self::Window, CanvasT)>
        where CanvasT: CanvasTrait<ShellClient = Self>;


    fn drop_window_with_canvas<CanvasT>(&self, window: Self::Window, canvas: CanvasT)
        -> Result<()>
        where CanvasT: CanvasTrait<ShellClient = Self>
    {
        canvas._drop(self, window)
    }


    fn is_subsystem_available(&self, subsystem: ShellSubsystem)
        -> bool;

    fn is_subsystem_enabled(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> bool;

    fn is_subsystem_forced(&self, window: &Self::Window, subsystem: ShellSubsystem)
        -> bool;

    fn set_subsystem_enabled(&self, window: &mut Self::Window, subsystem: ShellSubsystem, enabled: bool)
        -> Result<()>;

}


pub trait CanvasTrait : Sized {

    type ShellClient : ShellClientTrait;

    type CanvasInfo;

    fn _drop(self, client: &Self::ShellClient, window: <Self::ShellClient as ShellClientTrait>::Window)
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