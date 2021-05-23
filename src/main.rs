#[macro_use] extern crate arrayref;
extern crate once_cell;
extern crate sdl2;

mod nes;
use crate::nes::Nes;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = match args.len() {
        0 | 1 => panic!("rom file path argument is not found."),
        2 => &args[1],
        _ => panic!("too match arguments."),
    };

    let mut nes = Nes::new(rom_path);
    nes.run();
}
