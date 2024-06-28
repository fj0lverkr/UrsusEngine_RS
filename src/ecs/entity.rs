use super::{component::Component, manager::EntityGroup};

#[derive(PartialEq, Eq)]
pub struct Entity {
    is_active: bool,
    components: Vec<Component>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            is_active: true,
            components: Vec::new(),
        }
    }
    pub fn update(&mut self) {}

    pub fn draw(&self) {}

    pub fn has_group(&self, group: &EntityGroup) -> bool {
        //TODO implement this
        false
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn has_component(&self, component: Component) -> bool {
        match self.components.iter().position(|c| *c == component) {
            Some(_) => true,
            None => false,
        }
    }
}
