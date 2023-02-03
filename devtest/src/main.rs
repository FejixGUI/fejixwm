#![allow(unused)]

extern crate fejixwm;
use fejixwm::{*, interface::{window_manip::*}};

const WID: WindowId = 0;

fn main() {
    let result = run();
    if let Err(error) = result {
        println!("FejixWM error: {error}");
    }
}


fn run() -> fejixwm::errors::Result<()> {

    let wm_info = WindowManagerInfo { name: "com.example.helloworld" };

    let window_info = WindowInfo {
        id: WID,
        flags: WindowFlags::empty(),
        size: PixelSize::new(800, 600),
        canvas_info: CanvasInfo::None
    };

    let mut wm = WindowManager::new(&wm_info)?;
    wm.new_window(&window_info)?;
    wm.set_visible(WID, true)?;
    wm.set_title(WID, "Привіт Rust!")?;
    
    wm.run(|wm: &mut WindowManager, event: events::AnyEvent| match event {
        events::AnyEvent::WindowEvent { event, .. } => match event {
            events::WindowEvent::Close => {
                wm.stop();
            }
            _ => {}
        }
        _ => {}
    });

    Ok(())
}