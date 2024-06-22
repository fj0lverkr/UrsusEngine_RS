use sdl2::event::Event;
use sdl2::{pixels::Color, EventPump};

use crate::camera_2d::Camera2D;
use crate::renderer::Renderer;

pub struct Game {
    is_debug: bool,
    is_running: bool,
    pub renderer: Renderer,
    event_pump: EventPump,
    camera: Camera2D,
}

impl Game {
    pub fn new(
        title: &str,
        window_width: u32,
        window_height: u32,
        fullscreen: bool,
        is_debug: bool,
    ) -> Result<Game, String> {
        let render_color = Color::WHITE;
        let renderer = Renderer::new(title, window_width, window_height, fullscreen, render_color)?;
        let event_pump = renderer.sdl_context.event_pump()?;
        let camera = Camera2D::new(0.0, 0.0, window_width as f32, window_height as f32);
        Ok(Game {
            is_debug,
            is_running: true,
            renderer,
            event_pump,
            camera,
        })
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn update(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                _ => {}
            }
        }
        self.renderer.draw()?;
        Ok(())
    }
}
