use sdl2::event::Event;

use crate::event_system::{event_handler::EventHandler, event_receiver::EventReceiver};

use super::input_controller::KeyboardController;

pub trait ComponentBehavior {
    fn init(&mut self);
    fn draw(&self);
    fn update(&mut self);
}

#[derive(PartialEq, Eq)]
pub enum Component {
    KeyboardController(KeyboardController),
    Transform(KeyboardController),
    Sprite(KeyboardController),
}

#[derive(PartialEq, Eq)]
pub struct BaseComponent {
    event_handlers: Vec<EventReceiver>,
}

impl BaseComponent {
    pub fn new() -> BaseComponent {
        BaseComponent {
            event_handlers: Vec::new(),
        }
    }

    pub fn subscribe_to_events(&mut self, r: EventReceiver) {
        self.event_handlers.push(r);
        println!("{:?}", self.event_handlers);
    }

    pub fn handle_event(&self, e: &Event) -> bool {
        for handler in &self.event_handlers {
            if handler.handle_event(e) {
                return true;
            }
        }
        false
    }
}
