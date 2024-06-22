use super::event_receiver::EventReceiver;
use sdl2::{event::Event, keyboard::Keycode};

pub struct KeyboardEventReceiver;

impl EventReceiver for KeyboardEventReceiver {
    fn handle_event(&self, e: Event) -> bool {
        match e {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::W => true,
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
