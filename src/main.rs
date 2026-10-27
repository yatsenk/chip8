mod constants;
mod emulator;

use minifb::{Key, KeyRepeat, ScaleMode, Window, WindowOptions};

fn main() {
    let _chip8 = emulator::Chip8::new();

    let mut buffer = vec![0u32; constants::WIDTH * constants::HEIGHT];

    let mut text = Text::new(buffer.clone(), 20, 20, 10, 0x0000FF00);

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
                Key::Key0 => text.draw(0),
                Key::Key1 => text.draw(1),
                Key::Key2 => text.draw(2),
                Key::Key3 => text.draw(3),
                Key::Key4 => text.draw(4),
                Key::Key5 => text.draw(5),
                Key::Key6 => text.draw(6),
                Key::Key7 => text.draw(7),
                Key::Key8 => text.draw(8),
                Key::Key9 => text.draw(9),
                Key::A => text.draw(10),
                Key::B => text.draw(11),
                Key::C => text.draw(12),
                Key::D => text.draw(13),
                Key::E => text.draw(14),
                Key::F => text.draw(15),
                _ => (),
            });

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}

struct Text {
    buffer: Vec<u32>,
    x: u8,
    y: u8,
    scale: u8,
    color: u32,
}

impl Text {
    fn new(buffer: Vec<u32>, x: u8, y: u8, scale: u8, color: u32) -> Self {
        Text {
            buffer,
            x,
            y,
            scale,
            color,
        }
    }

    fn draw(&mut self, key: u8) {
        for row in 0..5u8 {
            let bits = constants::FONTSET[(key * 5 + row) as usize];

            for col in 0..4 {
                if bits & (0x80 >> col) != 0 {
                    for sy in 0..self.scale {
                        for sx in 0..self.scale {
                            let px = self.x + col * self.scale + sx;
                            let py = self.y + row * self.scale + sy;

                            if px < constants::WIDTH as u8 && py < constants::HEIGHT as u8 {
                                self.buffer[py as usize * constants::WIDTH + px as usize] =
                                    self.color;
                            }
                        }
                    }
                }
            }
        }
    }
}
