use crate::{
    *
};


pub struct Window {

}

pub struct App {

}


pub struct WindowVisualParams {

}


impl AppTrait for App {
    fn new(_name: String) -> Result<Self> {
        unimplemented!()
    }

    fn run<F>(&self, _event_handler: F)
        where F: FnMut(&AnyEvent)
    {
        unimplemented!()
    }
}


impl WindowTrait for Window {
    fn new(
        _window_params: &WindowParams,
        _surface_provider: Box<dyn SurfaceProvider>
    ) -> Result<Window>
    {
        unimplemented!()
    }

    fn set_title(&mut self, _title: &str) {
        unimplemented!()
    }


    fn get_surface_boxed(&self) -> &Box<dyn Any> {
        unimplemented!()
    }

    fn get_size(&self) -> PixelSize {
        unimplemented!()
    }
}