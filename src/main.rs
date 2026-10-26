mod constants;
mod emulator;

use minifb::{Key, KeyRepeat, ScaleMode, Window, WindowOptions};

fn main() {
    let _chip8 = emulator::Chip8::new();

    let mut buffer = vec![0u32; constants::WIDTH * constants::HEIGHT];

    let mut window = Window::new(
        "Chip8 emulator - press ESC to exit",
        constants::WIDTH * 10,
        constants::HEIGHT * 10,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = window.get_size();
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
        }

        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::Key0 => println!("pressed 0"),
                Key::Key1 => println!("pressed 1"),
                Key::Key2 => println!("pressed 2"),
                Key::Key3 => println!("pressed 3"),
                Key::Key4 => println!("pressed 4"),
                Key::Key5 => println!("pressed 5"),
                Key::Key6 => println!("pressed 6"),
                Key::Key7 => println!("pressed 7"),
                Key::Key8 => println!("pressed 8"),
                Key::Key9 => println!("pressed 9"),
                Key::A => println!("pressed a"),
                Key::B => println!("pressed b"),
                Key::C => println!("pressed c"),
                Key::D => println!("pressed d"),
                Key::E => println!("pressed e"),
                Key::F => println!("pressed f"),
                _ => (),
            });

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}

fn draw_key(mut buffer: Vec<u32>, width: u16, height: u16, x: u8, y: u8, key: u8, scale: u8) {
    if key > 15 || scale == 0 {
        return;
    }
}
