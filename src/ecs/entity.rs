use super::manager::EntityGroup;

#[derive(PartialEq, Eq)]
pub struct Entity {
    is_active: bool,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { is_active: true }
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
}
