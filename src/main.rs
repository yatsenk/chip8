mod emulator;

use minifb::{Key, ScaleMode, Window, WindowOptions};

fn main() {
    let _chip8 = emulator::Chip8::new();

    let mut buffer = vec![0u32; emulator::WIDTH * emulator::HEIGHT];

    let mut window = Window::new(
        "Chip8 emulator - press ESC to exit",
        emulator::WIDTH * 10,
        emulator::HEIGHT * 10,
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
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
