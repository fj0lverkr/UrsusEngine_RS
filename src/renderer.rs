use sdl2::{
    pixels::Color,
    render::{BlendMode, WindowCanvas},
    ttf::Sdl2TtfContext,
    video::Window,
    Sdl,
};

pub struct Renderer {
    pub window: Window,
    pub canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
    pub sdl_context: Sdl,
}

impl Renderer {
    pub fn new(
        title: &str,
        window_width: u32,
        window_height: u32,
        fullscreen: bool,
        render_color: Color,
    ) -> Result<Renderer, String> {
        let sdl_context = sdl2::init()?;
        let video_subsys = sdl_context.video()?;

        let mut window_builder = video_subsys.window(title, window_width, window_height);

        if fullscreen {
            window_builder.fullscreen();
        } else {
            window_builder.position_centered();
        }

        let window = window_builder.opengl().build().map_err(|e| e.to_string())?;

        let mut canvas = window
            .clone()
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_draw_color(render_color);

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        Ok(Renderer {
            window,
            canvas,
            ttf_context,
            sdl_context,
        })
    }

    pub fn draw(&mut self) -> Result<(), String> {
        //temp render something
        self.canvas.clear();
        self.canvas.present();
        Ok(())
    }
}
