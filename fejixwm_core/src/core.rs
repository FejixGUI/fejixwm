use crate::{
    Result,
    events::EventHandler,
};


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
    Resizable       = 0b0000_0001,
    SmoothRedraw    = 0b0000_0010,
    TextInput       = 0b0000_0100,
}


/// Window numeric identifier defined by the user that is passed to the event handler
pub type WindowId = u32;


pub struct WindowParams {
    /// Window numeric identifier defined by the user that is passed to the event handler
    id: WindowId,
    flags: WindowFlags,
    size: PixelSize,
}


pub trait PlatformApiTrait {

    type App : AppTrait<PlatformApi = Self>;

    /// App reference, any suitable container (e.g. `Mutex<App>`, `RefCell<App>`, `&'static App`).
    /// However, thread-safe ones are preferred.
    type AppRef : AppRefTrait<PlatformApi = Self>;

    type Window : WindowTrait<PlatformApi = Self>;

    /// Platform-dependent surface-specific data that is used internally for window creation
    type WindowInternalVisualData;

}


pub trait AppTrait : Sized {

    type PlatformApi : PlatformApiTrait;

    fn new(name: String) -> Result<Self>;

    fn get_ref(&self) -> <Self::PlatformApi as PlatformApiTrait>::AppRef;

    /// Runs event loop while the event handler does not return [EventOutcome::Exit]
    fn run(&self, event_handler: EventHandler);

}


pub trait AppRefTrait : Sized {

    type PlatformApi : PlatformApiTrait;

    fn get_ref(&self) -> Self;

}


pub trait WindowTrait : Sized {

    type PlatformApi : PlatformApiTrait;


    /// Internal constructor used inside surface constructors
    fn new_internal(
        app: <Self::PlatformApi as PlatformApiTrait>::AppRef,
        params: WindowParams,
        visual_data: <Self::PlatformApi as PlatformApiTrait>::WindowInternalVisualData
    ) -> Result<Self>;


    fn get_app(&self) -> <Self::PlatformApi as PlatformApiTrait>::AppRef;

    fn get_id(&self) -> WindowId;

}


/// Structs that implement this trait can be made into trait objects because [Sized] is not required.
pub trait SurfaceGenericTrait {

    type PlatformApi : PlatformApiTrait;

    fn get_window(&self) -> <Self::PlatformApi as PlatformApiTrait>::Window;

    fn get_size(&self) -> PixelSize;

    fn resize(&self, size: PixelSize) -> Result<()>;

}


pub trait SurfaceTrait : SurfaceGenericTrait + Sized {

    type SurfaceParams;

    fn new(
        app: <Self::PlatformApi as PlatformApiTrait>::AppRef,
        window_params: WindowParams,
        surface_params: Self::SurfaceParams
    ) -> Result<Self>;

}