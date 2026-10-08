use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM: usize = 4096;

struct Chip8 {
    pc: u8,
    i: u8,
    v: [u8; 16],
    stack: [u8; RAM],
    keypad: [u8; 16],
}

fn main() {}
