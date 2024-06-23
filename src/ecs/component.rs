use crate::event_system::event_receiver::EventReceiver;
use sdl2::event::Event;

pub struct BaseComponent<T: EventReceiver> {
    event_handlers: Vec<T>,
}

impl<T> BaseComponent<T>
where
    T: EventReceiver,
{
    pub fn new() -> BaseComponent<T> {
        BaseComponent {
            event_handlers: Vec::new(),
        }
    }

    pub fn subscribe_to_events(&mut self, r: T) {
        self.event_handlers.push(r);
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
