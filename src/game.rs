use sdl2::event::Event;
use sdl2::{pixels::Color, EventPump};

use crate::camera_2d::Camera2D;
use crate::ecs::input_controller::KeyboardController;
use crate::renderer::Renderer;

pub struct Game {
    is_debug: bool,
    is_running: bool,
    pub renderer: Renderer,
    event_pump: EventPump,
    camera: Camera2D,
    keyboard_controller: KeyboardController,
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
        let keyboard_controller = KeyboardController::new();
        Ok(Game {
            is_debug,
            is_running: true,
            renderer,
            event_pump,
            camera,
            keyboard_controller,
        })
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn update(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                self.is_running = false;
                break;
            }
            //here we can cascade through other EventReceivers and check if they have handled the
            //event, see Game.cpp in the original project.
            if self.keyboard_controller.component.handle_event(&event) {
                println!("Keyboard event handled correctly");
            } else {
                println!("Unhandled event {:?}", event);
            }
        }
        self.renderer.draw()?;
        Ok(())
    }
}
