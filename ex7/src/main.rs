mod controllers;
mod emulator;
mod plotter;
use std::sync::mpsc;

use plotter::init_window;

fn main() {
    let (tx, rx) = mpsc::channel::<f32>();
    let _ = std::thread::spawn(move || {
        let serial_port = emulator::init(tx);
        controllers::controller_main(serial_port);
    });
    init_window(rx);
}
