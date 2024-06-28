use super::event_handler::EventHandler;
use sdl2::{event::Event, keyboard::Keycode};

#[derive(Debug, PartialEq, Eq)]
pub struct KeyboardEventReceiver;

impl EventHandler for KeyboardEventReceiver {
    fn handle_event(&self, e: &Event) -> bool {
        match *e {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::W => {
                    println!("W key down");
                    true
                }
                Keycode::A => true,
                Keycode::S => true,
                Keycode::D => true,
                _ => false,
            },
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::W => true,
                Keycode::A => true,
                Keycode::S => true,
                Keycode::D => true,
                _ => false,
            },
            _ => false,
        }
    }
}
