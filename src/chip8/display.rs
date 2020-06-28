use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Display {
    pub canvas: WindowCanvas,
    pub screen: [[u8; 32]; 64],
    scale: u32,
    should_draw: bool,
}

impl Display {
    pub fn new(ctx: &sdl2::Sdl, window_scale: u32) -> Self {
        let video = ctx.video().unwrap();
        let window = video
            .window("CHIP-8 Emulator", 64 * window_scale, 32 * window_scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.clear();
        canvas.present();

        Display {
            canvas,
            screen: [[0; 32]; 64],
            scale: window_scale,
            should_draw: false,
        }
    }

    pub fn set_should_draw(&mut self, should_draw: bool) {
        self.should_draw = should_draw
    }

    pub fn should_draw(&self) -> bool {
        self.should_draw
    }

    pub fn clear_screen(&mut self) {
        for x in 0..self.screen.len() {
            for y in 0..self.screen[x].len() {
                self.screen[x][y] = 0;
            }
        }
    }

    pub fn draw_screen(&mut self) {
        for x in 0..self.screen.len() {
            for y in 0..self.screen[x].len() {
                let x_pos = x as u32 * self.scale;
                let y_pos = y as u32 * self.scale;

                self.canvas.set_draw_color(if self.screen[x][y] == 1 {
                    Color::WHITE
                } else {
                    Color::BLACK
                });
                let _ = self.canvas.fill_rect(Rect::new(
                    x_pos as i32,
                    y_pos as i32,
                    self.scale,
                    self.scale,
                ));
            }
        }
        self.canvas.present();
    }
}
