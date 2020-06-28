use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Display {
    pub canvas: WindowCanvas,
    pub screen: [[bool; 64]; 32],
    scale: u32,
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
            screen: [[false; 64]; 32],
            scale: window_scale,
        }
    }

    pub fn clear_screen(&mut self) {
        for x in 0..self.screen.len() {
            for y in 0..self.screen[x].len() {
                self.screen[x][y] = false;
            }
        }
    }

    pub fn draw_screen(&mut self) {
        for x in 0..self.screen.len() {
            for y in 0..self.screen[x].len() {
                let x_pos = x as u32 * self.scale;
                let y_pos = y as u32 * self.scale;

                self.canvas.set_draw_color(if self.screen[x][y] {
                    Color::WHITE
                } else {
                    Color::BLACK
                });
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, self.scale, self.scale));
            }
        }
        self.canvas.present();
    }
}
