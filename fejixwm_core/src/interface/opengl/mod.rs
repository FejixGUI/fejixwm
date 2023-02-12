use crate::{
    errors::Result,
    *
};

use libc::c_void;


pub struct OpenglInfo {
    pub major_version: u8,
    pub minor_version: u8,
    pub compatibility_flag: bool,

    pub red_bits: u8,
    pub green_bits: u8,
    pub blue_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,

    pub multisamples: u8,

    /// Set to 2 for double-buferring
    pub framebuffers: u8,
}


pub trait OpenglCanvasTrait : CanvasTrait {

    fn make_current(
        &self,
        client: &Self::ShellClient,
        window: &mut <Self::ShellClient as ShellClientTrait>::Window
    ) -> Result<()>;

    /// Returns `GraphicsApiFailed` if the context is not current
    fn swap_buffers(
        &self,
        client: &Self::ShellClient,
        window: &mut <Self::ShellClient as ShellClientTrait>::Window
    ) -> Result<()>;

    /// Returns `GraphicsApiFailed` if the context is not current.
    /// Returns `InvalidArgument` if the function does not exist.
    fn load_function(
        &self,
        client: &Self::ShellClient,
        window: &<Self::ShellClient as ShellClientTrait>::Window,
        function_name: &str
    ) -> Result<*const c_void>;

    /// Returns `GraphicsApiFailed` if the context is not current.
    fn is_extension_supported(
        &self,
        client: &Self::ShellClient,
        window: &<Self::ShellClient as ShellClientTrait>::Window,
        extension_name: &str
    ) -> Result<bool>;

}