#![allow(unused)]
use core::panic;

use minifb::{
    Key::{self, P},
    Window, WindowOptions,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM: usize = 4096;

const FONTSET: [u16; 80] = [
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
    gfx: [bool; WIDTH * HEIGHT],
    opcode: u16,
    pc: u16,
    i: u16,
    v: [u16; 16],
    sp: u16,
    stack: [u16; 16],
    memory: [u16; RAM],
    keypad: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    fn new() -> Self {
        let mut memory: [u16; RAM] = [0; RAM];
        memory[..80].copy_from_slice(&FONTSET);

        Self {
            gfx: [false; WIDTH * HEIGHT],
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
    fn load(&mut self, buffer: &[u16]) {
        let end = buffer.len();
        self.memory[512..end].copy_from_slice(buffer);
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
        self.opcode = self.memory[self.pc as usize] << 8 | self.memory[(self.pc + 1) as usize];

        match self.opcode & 0xF000 {
            // 0xA000: Sets I to the address NNN
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            // 0xB000: Jumps to the address NNN plus V0
            0xB000 => {}
            // 0x1000: Jumps to address NNN
            0x1000 => {}
            // 0x2000: Calls subroutine at NNN
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            0x0000 => {
                match self.opcode & 0x000F {
                    // 0x00E0: Clears the screen
                    0x0000 => {
                        self.gfx = [false; WIDTH * HEIGHT];
                        self.pc += 2;
                    }
                    // 0x00EE: Returns from subroutine
                    0x000E => {}
                    _ => panic!("unimplemented opcode"),
                }
            }
            0x8000 => {
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let y = ((self.opcode & 0x00F0) >> 4) as usize;

                match self.opcode & 0x000F {
                    // Sets VX to the value of VY
                    0x0000 => {
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }
                    0x0001 => {
                        self.v[x] |= self.v[y];
                        self.pc += 2;
                    }
                    0x0002 => {
                        self.v[x] &= self.v[y];
                        self.pc += 2;
                    }
                    0x0003 => {
                        self.v[x] ^= self.v[y];
                        self.pc += 2;
                    }
                    0x0004 => {
                        self.v[x] += self.v[y];
                        self.pc += 2;
                    }
                    0x0005 => {
                        self.v[x] -= self.v[y];
                        self.pc += 2;
                    }
                    0x0006 => {
                        self.v[x] >>= 1;
                        self.pc += 2;
                    }
                    0x0007 => {
                        self.v[x] = self.v[y] - self.v[x];
                        self.pc += 2;
                    }
                    0x000E => {
                        self.v[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => panic!("unimplemented opcode"),
                }
            }
            _ => panic!("unimplemented opcode"),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }
}

fn main() {
    let _chip8 = Chip8::new();
}
