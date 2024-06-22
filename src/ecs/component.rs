use crate::event_system::event_receiver::EventReceiver;

pub struct Component {
    pub event_handlers: Vec<Box<dyn EventReceiver>>,
}
