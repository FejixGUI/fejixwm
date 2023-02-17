#![allow(unused)]

extern crate fejixwm;
use fejixwm::{*, interface::{window_manip::*}};

const WID: WindowId = 0;

fn main() {
    if let Err(error) = run() {
        println!("FejixWM error: {error}");
    }
}


fn run() -> fejixwm::errors::Result<()> {

    let client = ShellClient::new(&ShellClientInfo {
        id: "dev.fejix.fejixwm.devtest",
        subsystems: &[]
    })?;

    

    Ok(())
}