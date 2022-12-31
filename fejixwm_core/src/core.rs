/*!
 * An app is a global singleton that represents a client of the operating system's graphical environment.
 * 
 * A window is a rectangular graphical surface that accepts user input and integrates into the operating system's
 * graphical environment.
 */

pub use crate::events::*;
pub use std::any::Any;

use std::ops::FnMut;
use crate::Result;


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


/* pub trait WmApi {
    type App;
    type Window;

    /// Platform-dependent SurfaceController-specific data that is used internally for window creation
    type WindowInternalVisualData;
}


pub trait AppTrait : Sized {
    type WmApi : WmApi;

    fn new(name: String) -> Result<Self>;

    fn run<F>(&self, event_handler: F)
        where F: FnMut(
            Option<&<<Self as AppTrait>::WmApi as WmApi>::Window>,
            &Event
        );
}


pub trait WindowTrait : Sized {
    type WmApi : WmApi;

    fn new(
        window_params: &WindowParams<WmApiT = Self::WmApi>,
        surface_provider: Box<dyn SurfaceController<WmApi = Self::WmApi>>
    ) -> Result<Self>;

    fn with<SurfaceApiT>(
        window_params: &WindowParams,
        surface_params: &SurfaceApiT::SurfaceParams
    ) -> Result<Self>
        where SurfaceApiT : SurfaceApi<WmApi = Self::WmApi>
    {
        SurfaceApiT::new_controller_boxed(surface_params)
            .and_then(|surface_provider| Self::new(window_params, surface_provider))
    }

    fn set_title(&self, title: &str) -> Result<()>;

    fn get_size(&self) -> PixelSize;

    fn get_surface_boxed(&self) -> &Box<dyn Any>;

    fn get_surface<SurfaceT>(&self) -> Option<&SurfaceT>
        where SurfaceT : 'static
    {
        self.get_surface_boxed().downcast_ref::<SurfaceT>()
    }

}


pub struct WindowParams<'a, WmApiT : WmApi> {
    app: &'a WmApiT::App,
    size: PixelSize,
    flags: WindowFlags,
}


/* pub trait SurfaceApi {
    type SurfaceProvider : SurfaceProvider + 'static;
    type SurfaceParams;
    type SurfaceController : SurfaceController;


    fn new_provider(
        params: &Self::SurfaceParams
    ) -> Result<Self::SurfaceProvider>;


    fn new_boxed(
        params: &Self::SurfaceParams
    ) -> Result<Box<dyn SurfaceProvider>>
    {
        Self::new_provider(params)
            .and_then(|provider| Ok(Box::new(provider) as Box<dyn SurfaceProvider>))
    }
}


pub trait SurfaceProvider {
    fn new_visual_params(&self) -> Result<WindowVisualParams>;

    fn drop_visual_params(&self, surface_params: WindowVisualParams);

    fn new_surface_boxed(&self, window: &Window) -> Result<Box<dyn Any>>;

    fn drop_surface(&self, window: &Window, surface: Box<dyn Any>);
} */




pub trait SurfaceApi {
    type WmApi : WmApi;
    type Surace;
    type SurfaceController : SurfaceController<WmApi = Self::WmApi>;
    type SurfaceParams;

    fn new_controller(
        params: &Self::SurfaceParams
    ) -> Result<Self::SurfaceController>;


    fn new_controller_boxed(
        params: &Self::SurfaceParams
    ) -> Result<Box<dyn SurfaceController<WmApi = Self::WmApi>>>
    {
        Self::new_controller(params)
            .and_then(|controller| Ok(Box::new(controller) as Box<dyn SurfaceController<WmApi = Self::WmApi>>))
    }
}

pub trait SurfaceController : 'static {
    type WmApi : WmApi;

    fn new_window_visuals(
        &self
    ) -> Result<<<Self as SurfaceController>::WmApi as WmApi>::WindowInternalVisualData>;

    fn drop_window_visuals(
        &self,
        surface_params: <<Self as SurfaceController>::WmApi as WmApi>::WindowInternalVisualData
    );

    fn new_surface_boxed(
        &self,
        window: &<<Self as SurfaceController>::WmApi as WmApi>::Window
    ) -> Result<Box<dyn Any>>;

    fn drop_surface(
        &self,
        window: &<<Self as SurfaceController>::WmApi as WmApi>::Window, 
        surface: Box<dyn Any>
    );
} */


/// Window Management API trait
pub trait WmApiTrait {

    type App : AppTrait;

    type Window : WindowTrait;

    /// Platform-dependent SurfaceController-specific data that is used internally for window creation
    type WindowInternalVisualData;

}


pub trait AppTrait : Sized {

    fn new(name: String) -> Result<Self>;

}


pub trait WindowTrait : Sized {


    
}