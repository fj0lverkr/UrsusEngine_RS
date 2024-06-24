use crate::event_system::{
    event_receiver::EventReceiver, keyboard_event_receiver::KeyboardEventReceiver,
};

use super::{component::BaseComponent, entity::Entity};

pub struct KeyboardController {
    pub component: BaseComponent,
    //sprite component
    //transform component
}

impl KeyboardController {
    pub fn new(controlled_entity: Entity) -> KeyboardController {
        let mut component = BaseComponent::new(controlled_entity);
        component.subscribe_to_events(EventReceiver::Keyboard(KeyboardEventReceiver {}));
        KeyboardController { component }
    }
}
