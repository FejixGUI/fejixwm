#![allow(unused)]

extern crate fejixwm;
use fejixwm::{*, interface::{window_manip::*}};

const WID: WindowId = 123;

fn main() {
    let result = run();
    if let Err(error) = result {
        println!("FejixWM error: {error}");
    }
}


fn run() -> Result<()> {

    let wm_info = WindowManagerInfo { name: "com.example.helloworld".to_string() };

    let window_info = WindowInfo {
        id: WID,
        flags: WindowFlags::empty() | WindowFlags::SMOOTH_REDRAW | WindowFlags::TEXT_INPUT,
        size: PixelSize::new(800, 600),
        canvas_info: CanvasInfo::None
    };

    let mut wm = WindowManager::new(&wm_info)?;
    wm.new_window(&window_info)?;
    wm.set_visible(WID, true)?;
    wm.set_title(WID, "Привіт Rust!")?;
    std::thread::sleep(std::time::Duration::from_millis(3000));

    Ok(())
}