use super::{
    component::{Component, ComponentBehavior},
    manager::EntityLabel,
};

#[derive(PartialEq, Eq)]
pub struct Entity {
    pub is_active: bool,
    pub components: Vec<Component>,
    pub labels: Vec<EntityLabel>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            is_active: true,
            components: Vec::new(),
            labels: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        for c in &mut self.components {
            c.update();
        }
    }

    pub fn draw(&self) {}

    pub fn add_label(&mut self, label: EntityLabel) {
        if !self.has_label(&label) {
            self.labels.push(label);
        }
    }

    pub fn has_label(&self, label: &EntityLabel) -> bool {
        self.labels.iter().any(|l| l == label)
    }

    pub fn add_component(&mut self, component: Component) {
        if !self.has_component(&component) {
            self.components.push(component);
        }
    }

    pub fn has_component(&self, component: &Component) -> bool {
        self.components.iter().any(|c| c == component)
    }
}
