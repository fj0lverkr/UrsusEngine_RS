use std::collections::HashMap;

use super::entity::Entity;

const MAXGROUPS: usize = 32;

pub enum EntityGroup {
    Player,
}

pub struct Manager<'a> {
    entities: Vec<Entity>,
    grouped_entities: HashMap<EntityGroup, Vec<&'a Entity>>,
}

impl<'a> Manager<'_> {
    pub fn new() -> Manager<'a> {
        Manager {
            entities: Vec::new(),
            grouped_entities: HashMap::with_capacity(MAXGROUPS),
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
        for (entity_group, entities_in_group) in &mut self.grouped_entities {
            entities_in_group.retain(|&e| e.has_group(entity_group) && e.is_active());
        }
        self.entities.retain(|e| e.is_active());
    }
}
