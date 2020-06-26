use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

const CHIP8_NUM_KEYS: usize = 16;

pub struct Input {
    event_pump: EventPump,
    keys: [bool; CHIP8_NUM_KEYS],
}

impl Input {
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        Input {
            event_pump: ctx.event_pump().unwrap(),
            keys: [false; CHIP8_NUM_KEYS],
        }
    }

    pub fn is_pressed(&self, key_num: usize) -> bool {
        self.keys[key_num]
    }

    pub fn get_inputs(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.keys[0x1] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => self.keys[0x2] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => self.keys[0x3] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => self.keys[0x4] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.keys[0x5] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => self.keys[0x6] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.keys[0x7] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.keys[0x8] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.keys[0x9] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => self.keys[0xA] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => self.keys[0x0] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => self.keys[0xB] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => self.keys[0xC] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => self.keys[0xD] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => self.keys[0xE] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => self.keys[0xF] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.keys[0x1] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => self.keys[0x2] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => self.keys[0x3] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => self.keys[0x4] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => self.keys[0x5] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => self.keys[0x6] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => self.keys[0x7] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.keys[0x8] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => self.keys[0x9] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => self.keys[0xA] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => self.keys[0x0] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => self.keys[0xB] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => self.keys[0xC] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => self.keys[0xD] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => self.keys[0xE] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    ..
                } => self.keys[0xF] = false,
                _ => {}
            }
        }
    }
}
