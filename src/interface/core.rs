pub use crate::{
    interface::events::*,
    platform::{App, Window}
};

pub use std::{
    any::Any
};

use crate::{
    platform::WindowVisualParams
};

use std::{
    ops::FnMut,
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
    SmoothRedraw    = 0b0000_0001,
    TextInput       = 0b0000_0010,
}


pub trait AppTrait {
    fn new(name: String) -> Self;

    fn run<F>(&self, event_handler: F)
        where F: FnMut(&AnyEvent);
}


pub trait WindowTrait {
    fn new(window_params: &WindowParams, surface_provider: Box<dyn SurfaceProvider>) -> Window;

    fn set_title(&mut self, title: &str);

    fn get_size(&self) -> PixelSize;

    fn get_surface_boxed(&self) -> &Box<dyn Any>;


    fn with<SurfaceApiT>(window_params: &WindowParams, surface_params: &SurfaceApiT::Params) -> Window
        where SurfaceApiT : SurfaceApi
    {
        Self::new(
            window_params,
            SurfaceApiT::new_boxed(surface_params)
        )
    }


    fn get_surface<SurfaceT>(&self) -> Option<&SurfaceT>
        where SurfaceT : 'static
    {
        self.get_surface_boxed().downcast_ref::<SurfaceT>()
    }
}


pub struct WindowParams<'a> {
    app: &'a App,
    size: PixelSize,
    flags: WindowFlags,
}


pub trait SurfaceApi {
    type Params;
    type Provider : SurfaceProvider + 'static;


    fn new(
        params: &Self::Params
    ) -> Self::Provider;


    fn new_boxed(
        params: &Self::Params
    ) -> Box<dyn SurfaceProvider>
    {
        Box::new(Self::new(params))
            as Box<dyn SurfaceProvider>
    }
}


pub trait SurfaceProvider {
    fn new_visual_params(&self) -> WindowVisualParams;

    fn drop_visual_params(&self, surface_params: WindowVisualParams);

    fn new_surface_boxed(&self, window: &Window) -> Box<dyn Any>;

    fn drop_surface(&self, window: &Window, surface: Box<dyn Any>);
}