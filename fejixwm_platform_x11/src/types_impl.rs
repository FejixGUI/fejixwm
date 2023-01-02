use crate::core;
use crate::core::traits::*;
use crate::types::*;

use std::{
    rc::Rc,
    cell::RefCell,
};


pub struct PlatformApi;


impl PlatformApiTrait for PlatformApi {
    type App = App;
    type AppRef = AppRef;
    type Window = Window;
    type WindowInternalVisualData = WindowInternalVisualData;
}



impl AppTrait for App {

    type PlatformApi = PlatformApi;

    fn new(name: String) -> core::Result<Self> {
        Ok(Self {
            app: Rc::new(RefCell::new(PlatformApp::new(name)?))
        })
    }

    fn get_ref(&self) -> AppRef {
        AppRef { app: Rc::clone(&self.app) }
    }

    fn run(&self, event_handler: core::events::EventHandler) {
        
    }

}



impl AppRefTrait for AppRef {

    type PlatformApi = PlatformApi;

    fn get_ref(&self) -> Self {
        Self { app: Rc::clone(&self.app) }
    }

}



impl WindowTrait for Window {

    type PlatformApi = PlatformApi;

    fn new_internal(
        app: AppRef,
        params: core::WindowParams,
        visual_data: WindowInternalVisualData
    ) -> core::Result<Self> {
        unimplemented!()
    }

    fn get_app(&self) -> AppRef {
        self.app.get_ref()
    }

    fn get_id(&self) -> core::WindowId {
        self.id
    }

}



impl PlatformApp {
    pub fn new(name: String) -> core::Result<Self> {
        unimplemented!()
    }
}