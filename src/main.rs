#![allow(unused)]
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM: usize = 4096;

const FONTSET: [usize; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Chip8 {
    opcode: usize,
    pc: usize,
    i: usize,
    v: [usize; 16],
    sp: usize,
    stack: [usize; 16],
    memory: [usize; RAM],
    keypad: [usize; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    fn new() -> Self {
        let mut memory: [usize; RAM] = [0; RAM];
        memory[..80].copy_from_slice(&FONTSET);

        Self {
            opcode: 0,
            pc: 0x200,
            i: 0,
            v: [0; 16],
            sp: 0,
            stack: [0; 16],
            memory,
            keypad: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    // 0x200 == 512
    fn load(&mut self, buffer: &[usize]) {
        self.memory[512..].copy_from_slice(buffer);
    }

    /// example
    ///
    /// 0xA2       0xA2 << 8 = 0xA200   HEX
    /// 10100010   1010001000000000     BIN
    ///
    /// bitwise or ( | ) operation
    ///
    /// 1010001000000000                // 0xA200
    ///         11110000                // 0xF0 (0x00F0)
    /// ----------------
    /// 1010001011110000                // 0xA2F0
    fn emulate(&mut self) {
        self.opcode = self.memory[self.pc] << 8 | self.memory[self.pc + 1];

        match self.opcode & 0xF000 {
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            0x2000 => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            _ => todo!(),
        }
    }
}

fn main() {
    let _chip8 = Chip8::new();
}
