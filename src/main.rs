#![allow(unused)]
use std::io;
use std::path::Path;
use std::{fs::File, io::Read};

use minifb::{
    Key::{self, P},
    Window, WindowOptions,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM: usize = 4096;

const FONTSET: [u8; 80] = [
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
    gfx: [u8; WIDTH * HEIGHT],
    opcode: u16,
    pc: u16,
    i: u16,
    v: [u8; 16],
    sp: u16,
    stack: [u16; 16],
    memory: [u8; RAM],
    keypad: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    fn new() -> Self {
        let mut memory: [u8; RAM] = [0; RAM];
        memory[..80].copy_from_slice(&FONTSET);

        Self {
            gfx: [0u8; WIDTH * HEIGHT],
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
    fn load<P: AsRef<Path>>(&mut self, path_to_file: P) {
        let mut f = match File::open(path_to_file) {
            Ok(f) => f,
            Err(e) => panic!("cannot open a file: {}", e),
        };

        let mut buffer = vec![];
        f.read_to_end(&mut buffer);

        self.memory[512..buffer.len()].copy_from_slice(&buffer);
    }

    fn emulate(&mut self) {
        self.opcode =
            (self.memory[self.pc as usize] << 8 | self.memory[(self.pc + 1) as usize]) as u16;

        match self.opcode & 0xF000 {
            // 0x00__
            0x0000 => {
                match self.opcode & 0x000F {
                    // 0x00E0: Clears the screen
                    0x0000 => {
                        self.gfx = [0; WIDTH * HEIGHT];
                        self.pc += 2;
                    }
                    // 0x00EE: Returns from subroutine
                    0x000E => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.pc += 2;
                    }
                    _ => panic!("unimplemented opcode"),
                }
            }
            // Jumps to address NNN
            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            }
            // 0x2000: Calls subroutine at NNN
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            // Skips the next instruction if VX
            // equals NN (usually the next instruction
            // is a jump to skip a code block)
            0x3000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x0FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // Skips the next instruction if VX
            // does not equal NN (usually the next instruction
            // is a jump to skip a code block)
            0x4000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // Skips the next instruction if VX equals VY
            // (usually the next instruction is a jump to
            // skip a code block)
            0x5000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize]
                    == self.v[((self.opcode & 0x00F0) >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // Sets VX to NN
            0x6000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            // Adds NN to VX (carry flag is not changed)
            0x7000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] += (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            // 8XYN
            0x8000 => {
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let y = ((self.opcode & 0x00F0) >> 4) as usize;

                match self.opcode & 0x000F {
                    // Sets VX to the value of VY
                    0x0000 => {
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }
                    // Sets VX to VX or VY. (bitwise OR operation)
                    0x0001 => {
                        self.v[x] |= self.v[y];
                        self.pc += 2;
                    }
                    // Sets VX to VX and VY. (bitwise AND operation)
                    0x0002 => {
                        self.v[x] &= self.v[y];
                        self.pc += 2;
                    }
                    // Sets VX to VX xor VY
                    0x0003 => {
                        self.v[x] ^= self.v[y];
                        self.pc += 2;
                    }
                    // Adds VY to VX. VF is set to 1 when there's an overflow,
                    // and to 0 when there is not
                    0x0004 => {
                        self.v[x] += self.v[y];
                        // Vy > 255 - Vx
                        if self.v[y] > 0xFF - self.v[x] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.pc += 2;
                    }
                    // VY is subtracted from VX. VF is set to 0
                    // when there's an underflow, and 1 when there is not.
                    // (i.e. VF set to 1 if VX >= VY and 0 if not)
                    0x0005 => {
                        if self.v[x] >= self.v[y] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] -= self.v[y];
                        self.pc += 2;
                    }
                    // Shifts VX to the right by 1, then stores the least
                    // significant bit of VX prior to the shift into VF
                    0x0006 => {
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                        self.pc += 2;
                    }
                    // Sets VX to VY minus VX. VF is set to 0 when
                    // there's an underflow, and 1 when there is not.
                    // (i.e. VF set to 1 if VY >= VX)
                    0x0007 => {
                        if self.v[y] >= self.v[x] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] = self.v[y] - self.v[x];
                        self.pc += 2;
                    }
                    // Shifts VX to the left by 1, then sets VF to 1
                    // if the most significant bit of VX prior to that
                    // shift was set, or to 0 if it was unset
                    0x000E => {
                        self.v[0xF] = self.v[x] >> 7;
                        self.v[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => panic!("unimplemented opcode"),
                }
            }
            // Skips the next instruction if VX does not
            // equal VY. (Usually the next instruction is
            // a jump to skip a code block)
            0x9000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize]
                    != self.v[((self.opcode & 0x00F0) >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // 0xA000: Sets I to the address NNN
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            // 0xB000: Jumps to the address NNN plus V0
            0xB000 => {
                self.pc = (self.opcode & 0x0FFF) + self.v[0x0] as u16;
            }
            // Sets VX to the result of a bitwise and
            // operation on a random number
            // (Typically: 0 to 255) and NN
            0xC000 => {
                // todo: random number instead of 42
                self.v[((self.opcode & 0x0F00) >> 8) as usize] = 42 & (self.opcode & 0x00FF) as u8;
            }
            // Draws a sprite at coordinate (VX, VY) that
            // has a width of 8 pixels and a height of N pixels.
            // Each row of 8 pixels is read as bit-coded
            // starting from memory location I; I value does not
            // change after the execution of this instruction.
            // As described above, VF is set to 1 if any screen
            // pixels are flipped from set to unset when
            // the sprite is drawn, and to 0 if that does not happen
            0xD000 => {}
            // EX__
            0xE000 => {
                match self.opcode & 0x00FF {
                    // Skips the next instruction if the key stored
                    // in VX(only consider the lowest nibble)
                    // is pressed (usually the next instruction
                    // is a jump to skip a code block)
                    0x009E => {
                        if self.keypad[self.v[((self.opcode & 0x0F00) >> 8) as usize] as usize] != 0
                        {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    // Skips the next instruction if the key
                    // stored in VX(only consider the lowest nibble)
                    // is not pressed (usually the next instruction
                    // is a jump to skip a code block)
                    0x00A1 => {
                        if self.keypad[self.v[((self.opcode & 0x0F00) >> 8) as usize] as usize] == 0
                        {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("unimplemented opcode"),
                }
            }
            // FX__
            0xF000 => {
                match self.opcode & 0x00FF {
                    // Sets VX to the value of the delay timer
                    0x0007 => {
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] = self.delay_timer;
                    }
                    // A key press is awaited, and then stored in VX
                    // (blocking operation, all instruction halted until next key event,
                    // delay and sound timers should continue processing)
                    0x000A => {
                        let mut key_pressed = false;

                        for i in 0..16 {
                            if self.keypad[i] != 0 {
                                self.v[((self.opcode & 0x0F00) >> 8) as usize] = i as u8;
                                key_pressed = true;
                            }
                        }

                        if !key_pressed {
                            return;
                        }

                        self.pc += 2;
                    }
                    // Sets the delay timer to VX
                    0x0015 => {
                        self.delay_timer = self.v[((self.opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    }
                    // Sets the sound timer to VX
                    0x0018 => {
                        self.sound_timer = self.v[((self.opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    }
                    // Adds VX to I. VF is not affected
                    0x001E => {
                        // ???
                        self.i += self.v[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                        self.pc += 2;
                    }
                    // Sets I to the location of the sprite
                    // for the character in VX (only consider
                    // the lowest nibble). Characters 0-F
                    // (in hexadecimal) are represented by a 4x5 font
                    0x0029 => {
                        self.i = (self.v[((self.opcode & 0x0F00) >> 8) as usize] * 0x5) as u16;
                        self.pc += 2;
                    }
                    // Stores the binary-coded decimal representation
                    // of VX, with the hundreds digit in memory at location
                    // in I, the tens digit at location I+1, and
                    // the ones digit at location I+2
                    0x0033 => {
                        self.memory[self.i as usize] =
                            self.v[((self.opcode & 0x0F00) >> 8) as usize] / 100;
                        self.memory[(self.i + 1) as usize] =
                            (self.v[((self.opcode & 0x0F00) >> 8) as usize] / 10) % 10;
                        self.memory[(self.i + 2) as usize] =
                            self.v[((self.opcode & 0x0F00) >> 8) as usize] % 10;
                        self.pc += 2;
                    }
                    // Stores from V0 to VX (including VX) in memory, starting
                    // at address I. The offset from I is increased by 1 for
                    // each value written, but I itself is left unmodified
                    0x0055 => {
                        for i in 0..=((self.opcode & 0x0F00) >> 8) as usize {
                            self.memory[(self.i as usize) + 1] = self.v[i];
                        }
                        // ???
                        self.i += ((self.opcode & 0x0F00) >> 8) + 1;
                        self.pc += 2;
                    }
                    // Fills from V0 to VX (including VX) with values from memory,
                    // starting at address I. The offset from I is increased by 1
                    // for each value read, but I itself is left unmodified
                    0x0065 => {
                        for i in 0..=((self.opcode & 0x0F00) >> 8) as usize {
                            self.v[i] = self.memory[(self.i as usize) + 1];
                        }
                        // ???
                        self.i += ((self.opcode & 0x0F00) >> 8) + 1;
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

    let mut window = Window::new(
        "Chip8 emulator",
        WIDTH * 10,
        HEIGHT * 10,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() {
        window.update();
    }
}
