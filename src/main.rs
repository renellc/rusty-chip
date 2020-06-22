mod cpu;
mod display;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut display = display::Display::new(ctx.video().unwrap(), 10);
    let mut event_pump = ctx.event_pump().unwrap();

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                _ => {}
            }
        }

        display.canvas.present();

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
