mod constants;
mod emulator;

use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

fn main() {
    let mut chip8 = emulator::Chip8::new();

    let mut buffer = vec![0u32; constants::WIDTH * constants::HEIGHT];
    let mut window = Window::new(
        "Chip8 emulator - press ESC to exit",
        constants::WIDTH,
        constants::HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X16,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    if let Err(e) = chip8.load("roms/PONG") {
        eprintln!("error occured: {}", e);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..10 {
            chip8.emulate_cycle();
        }

        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::Key0 => chip8.keypad[0] = 1,
                Key::Key1 => chip8.keypad[1] = 1,
                Key::Key2 => chip8.keypad[2] = 1,
                Key::Key3 => chip8.keypad[3] = 1,
                Key::Key4 => chip8.keypad[4] = 1,
                Key::Key5 => chip8.keypad[5] = 1,
                Key::Key6 => chip8.keypad[6] = 1,
                Key::Key7 => chip8.keypad[7] = 1,
                Key::Key8 => chip8.keypad[8] = 1,
                Key::Key9 => chip8.keypad[9] = 1,
                Key::A => chip8.keypad[10] = 1,
                Key::B => chip8.keypad[11] = 1,
                Key::C => chip8.keypad[12] = 1,
                Key::D => chip8.keypad[13] = 1,
                Key::E => chip8.keypad[14] = 1,
                Key::F => chip8.keypad[15] = 1,
                _ => (),
            });

        window.get_keys_released().iter().for_each(|key| match key {
            Key::Key0 => chip8.keypad[0] = 0,
            Key::Key1 => chip8.keypad[1] = 0,
            Key::Key2 => chip8.keypad[2] = 0,
            Key::Key3 => chip8.keypad[3] = 0,
            Key::Key4 => chip8.keypad[4] = 0,
            Key::Key5 => chip8.keypad[5] = 0,
            Key::Key6 => chip8.keypad[6] = 0,
            Key::Key7 => chip8.keypad[7] = 0,
            Key::Key8 => chip8.keypad[8] = 0,
            Key::Key9 => chip8.keypad[9] = 0,
            Key::A => chip8.keypad[10] = 0,
            Key::B => chip8.keypad[11] = 0,
            Key::C => chip8.keypad[12] = 0,
            Key::D => chip8.keypad[13] = 0,
            Key::E => chip8.keypad[14] = 0,
            Key::F => chip8.keypad[15] = 0,
            _ => (),
        });

        if chip8.draw_flag {
            chip8.draw_flag = false;

            for i in 0..2048 {
                let pixel = chip8.gfx[i] as u32;
                buffer[i] = (0x00FFFFFF * pixel) | 0xFF000000;
            }
        }

        window
            .update_with_buffer(&buffer, constants::WIDTH, constants::HEIGHT)
            .unwrap();
    }
}
