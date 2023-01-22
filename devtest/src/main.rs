#![allow(unused)]

extern crate fejixwm;
use fejixwm::*;

fn main() {
    let result = run();
    if let Err(error) = result {
        println!("FejixWM error: {error}");
    }
}


fn run() -> Result<()> {

    Ok(())
}