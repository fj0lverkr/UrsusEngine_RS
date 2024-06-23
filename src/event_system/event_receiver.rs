use super::{event_handler::EventHandler, keyboard_event_receiver::KeyboardEventReceiver};
use sdl2::event::Event;

#[derive(Debug)]
pub enum EventReceiver {
    // TODO create these for all SDL2 Event types
    Keyboard(KeyboardEventReceiver),
}

impl EventHandler for EventReceiver {
    fn handle_event(&self, e: &Event) -> bool {
        match self {
            EventReceiver::Keyboard(r) => r.handle_event(e),
        }
    }
}
