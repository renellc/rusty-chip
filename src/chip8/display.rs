use sdl2::render::WindowCanvas;

pub struct Display {
    pub canvas: WindowCanvas,
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

        Display { canvas }
    }
}
