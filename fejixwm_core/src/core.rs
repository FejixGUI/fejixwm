use crate::{
    Result,
    events::Event,
};

use std::ops::FnMut;


pub struct PixelSize {
    pub width: u32,
    pub height: u32
}

impl PixelSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

type WindowFlags = u32;

#[repr(u32)]
enum WindowFlag {
    SmoothRedraw    = 0b0000_0001,
    TextInput       = 0b0000_0010,
}


pub struct WindowParams {
    size: PixelSize,
    flags: WindowFlags
}


pub trait PlatformApiTrait {

    type App : AppTrait;

    /// App reference, any suitable container (e.g. `Mutex<App>`, `RefCell<App>`, `&'static App`).
    /// However, thread-safe ones are preferred.
    type AppRef : AppRefTrait;

    type Window : WindowTrait;

    /// Platform-dependent surface-specific data that is used internally for window creation
    type WindowInternalVisualData;

}


pub trait AppTrait : Sized {

    type PlatformApi : PlatformApiTrait;


    fn new(name: String) -> Result<Self>;

    fn get_ref(&self) -> <<Self as AppTrait>::PlatformApi as PlatformApiTrait>::AppRef;


    fn run<EventHandlerT>(
        &self,
        event_handler: EventHandlerT
    )
    where
        EventHandlerT : FnMut(
            Option<&<<Self as AppTrait>::PlatformApi as PlatformApiTrait>::Window>,
            Event
        );

}


pub trait AppRefTrait : Sized {

    type PlatformApi : PlatformApiTrait;

    fn get_ref(&self) -> Self;

    fn stop(&self);

}


pub trait WindowTrait : Sized {

    type PlatformApi : PlatformApiTrait;

    /// Internal constructor used inside surface constructors
    fn new_internal(
        app: <<Self as WindowTrait>::PlatformApi as PlatformApiTrait>::AppRef,
        params: WindowParams,
        visual_data: <<Self as WindowTrait>::PlatformApi as PlatformApiTrait>::WindowInternalVisualData
    ) -> Result<Self>;
    
}


/// Structs that implement this trait can be made into trait objects (because `Sized` is not required).
pub trait SurfaceBasicTrait {

    type PlatformApi : PlatformApiTrait;

    fn get_window(
        &self
    ) -> <<Self as SurfaceBasicTrait>::PlatformApi as PlatformApiTrait>::Window;

}


pub trait SurfaceApiTrait : Sized {

    type PlatformApi : PlatformApiTrait;
    
    type SurfaceParams;

    fn new(
        app: <<Self as SurfaceApiTrait>::PlatformApi as PlatformApiTrait>::AppRef,
        window_params: WindowParams,
        surface_params: Self::SurfaceParams
    ) -> Result<Self>;

}


pub trait SurfaceTrait : SurfaceBasicTrait + SurfaceApiTrait { }