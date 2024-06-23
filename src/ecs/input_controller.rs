use crate::event_system::keyboard_event_receiver::KeyboardEventReceiver;

use super::component::BaseComponent;

pub struct KeyboardController {
    pub component: BaseComponent<KeyboardEventReceiver>,
    //sprite component
    //transform component
}

impl KeyboardController {
    pub fn new() -> KeyboardController {
        let mut component = BaseComponent::new();
        component.subscribe_to_events(KeyboardEventReceiver {});
        KeyboardController { component }
    }
}
