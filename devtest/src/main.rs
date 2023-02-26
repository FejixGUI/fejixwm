#![allow(unused)]

extern crate fejixwm;
use fejixwm::{
    prelude::*,
    interface::{window_manip::*},
    implementation::null_canvas::NullCanvas
};

const WID: WindowId = 0;

fn main() {
    if let Err(error) = run() {
        println!("FejixWM error: {error}");
    }
}


fn run() -> fejixwm::errors::Result<()> {

    let client = ShellClient::new(&ShellClientInfo {
        id: "dev.fejix.fejixwm.devtest",
        subsystems: ShellSubsystem::list()
    })?;

    let (mut window, mut canvas) = NullCanvas::new(
        &client,
        &WindowInfo {
            size: PixelSize::new(800, 600),
        },
        &()
    )?;

    for subsystem in ShellSubsystem::list() {
        client.enable_subsystem(&mut window, subsystem.clone())?;
    }

    client.set_visible(&mut window, true)?;
    client.set_title(&mut window, "Привіт, Rust!")?;

    let mut i = 0;

    client.listen_to_events(|event: Option<&ShellMessage>, setting: &mut EventListeningSettings| {

    })?;

    canvas.drop(&client, window)?;


    Ok(())
}