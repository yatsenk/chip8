#![allow(unused)]
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM: usize = 4096;

struct Chip8 {
    opcode: u16,
    pc: u16,
    i: u8,
    v: [u8; 16],
    sp: u8,
    stack: [u8; 16],
    memory: [u8; RAM],
    keypad: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    fn new() -> Self {
        Self {
            opcode: 0,
            pc: 0x200,
            i: 0,
            v: [0; 16],
            sp: 0,
            stack: [0; 16],
            memory: [0; RAM],
            keypad: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

// https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

fn main() {
    let _chip8 = Chip8::new();
}
