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
        emu.emulate_cycle();
        emu.display.draw_screen();
        emu.input.get_inputs();

        if emu.input.should_quit() {
            break;
        }

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
