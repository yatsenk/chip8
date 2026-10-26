mod emulator;

use minifb::{
    Key::{self, P},
    Window, WindowOptions,
};

fn main() {
    let _chip8 = emulator::Chip8::new();

    let mut window = Window::new(
        "Chip8 emulator",
        emulator::WIDTH * 10,
        emulator::HEIGHT * 10,
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
