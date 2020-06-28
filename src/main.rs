mod chip8;

use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("No rom given");
    }

    let rom_path = args.get(1).unwrap().as_str();
    let mut emu = chip8::Chip8::new();
    emu.load_rom(rom_path);

    loop {
        emu.display.set_should_draw(false);
        emu.emulate_cycle();

        if emu.display.should_draw() {
            emu.display.draw_screen();
        }

        if emu.input.should_quit() {
            break;
        }

        emu.input.get_inputs();
        // thread::sleep(Duration::from_millis(1000 / 60));
    }
}
