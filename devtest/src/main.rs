#![allow(unused)]

extern crate fejixwm;
use fejixwm::{
    prelude::*,
    interface::{window_manip::*},
    implementation::null_canvas::NullCanvas
};

fn main() {
    if let Err(error) = run() {
        println!("FejixWM error: {error}");
    }
}


fn run() -> fejixwm::errors::Result<()> {

    let client = ShellClient::new(&ShellClientInfo {
        id: "dev.fejix.fejixwm.devtest",
        subsystems: ShellSubsystem::all()
    })?;

    let (mut window, mut canvas) = NullCanvas::new(
        &client,
        &WindowInfo {
            size: PixelSize::new(800, 600),
        },
        &()
    )?;

    // for subsystem in ShellSubsystem::all() {
    //     client.enable_subsystem(&mut window, subsystem.clone())?;
    // }

    client.set_visible(&mut window, true)?;
    client.set_title(&mut window, "Привіт, Rust!")?;

    client.post_message(Some(Box::new(123u32)))?;

    client.listen_to_messages(|message: Option<&ShellMessage>, settings: &mut ListeningSettings| {
        if message.is_none() {
            settings.behavior = ListeningBehavior::Await;
            return;
        }

        let message = message.unwrap();

        let maybe_window = if message.get_window_id() == Some(window.get_id()) {
            Some(&mut window)
        } else {
            None
        };

        client.process_message(message, maybe_window, |event: Event, window: Option<&mut Window>| {
            println!("{}", event);

            if let Event::WindowEvent(WindowEvent::Close) = event {
                settings.should_stop = true;
            } else if let Event::UserEvent(UserEvent { data }) = event {
                println!("Got data: {}", data.unwrap().downcast::<u32>().unwrap())
            }
        });
    })?;

    canvas.drop(&client, window)?;


    Ok(())
}