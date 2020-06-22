use sdl2::render::WindowCanvas;
use sdl2::VideoSubsystem;

pub struct Display {
    pub canvas: WindowCanvas,
}

impl Display {
    pub fn new(ctx: VideoSubsystem, window_scale: u32) -> Self {
        let window = ctx
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
