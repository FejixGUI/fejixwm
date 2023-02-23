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
            id: 123
        },
        &()
    )?;

    // FIXME throws Error::SubsystemForced
    // for subsystem in ShellSubsystem::list() {
    //     client.enable_subsystem(&mut window, subsystem.clone())?;
    // }

    client.set_visible(&mut window, true)?;
    client.set_title(&mut window, "Привіт, Rust!")?;

    client.process_events(&[&mut window], 
        &mut |client: &ShellClient, window: Option<&&mut Window>, event: Event| -> EventResponse {
            match event {
                Event::Close => EventResponse::EndProcessing,
                _ => EventResponse::WaitForEvents,
            }
        }
    )?;

    canvas.drop(&client, window)?;


    Ok(())
}