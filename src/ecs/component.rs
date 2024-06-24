use sdl2::event::Event;

use crate::event_system::{event_handler::EventHandler, event_receiver::EventReceiver};

use super::{entity::Entity, input_controller::KeyboardController};

pub enum Component {
    KeyboardControllerComponent(KeyboardController),
}

pub struct BaseComponent {
    event_handlers: Vec<EventReceiver>,
    entity: Entity,
}

impl BaseComponent {
    pub fn new(entity: Entity) -> BaseComponent {
        BaseComponent {
            event_handlers: Vec::new(),
            entity,
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
