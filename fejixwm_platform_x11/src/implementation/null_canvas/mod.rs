use crate::{
    *,
    errors::*,
    core::interface::null_canvas::NullCanvasTrait
};

pub struct NullCanvas;

impl NullCanvasTrait for NullCanvas {}

impl CanvasTrait for NullCanvas {

    type CanvasInfo = ();

    type ShellClient = X11ShellClient;

    type Window = <Self::ShellClient as ShellClientTrait>::Window;

    fn new(client: &Self::ShellClient, window_info: &WindowInfo, canvas_info: &Self::CanvasInfo)
    -> Result<(Self::Window, Self)> {
        let window = client.new_window(window_info, &client.get_default_visual_info())?;
        Ok((window, Self))
    }

    fn drop(self, client: &Self::ShellClient, window: Self::Window) -> Result<()> {
        client.drop_window(window)
    }

}
