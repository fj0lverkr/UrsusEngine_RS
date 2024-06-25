use super::entity::Entity;

const MAXGROUPS: usize = 32;

pub struct Manager<'a> {
    entities: Vec<Entity>,
    grouped_entities: Vec<Vec<&'a Entity>>,
}

impl<'a> Manager<'_> {
    pub fn new() -> Manager<'a> {
        Manager {
            entities: Vec::new(),
            grouped_entities: Vec::with_capacity(MAXGROUPS),
        }
    }

    pub fn update(&mut self) {
        for e in &mut self.entities {
            e.update();
        }
    }

    pub fn draw(&self) {
        for e in &self.entities {
            e.draw();
        }
    }

    pub fn refresh(&mut self) {
        for entity_group in &mut self.grouped_entities {}
    }
}
