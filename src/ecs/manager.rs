use std::collections::HashMap;

use super::entity::Entity;

const MAXGROUPS: usize = 32;

#[derive(Eq, PartialEq, Hash)]
pub enum EntityGroup {
    Player,
}

#[derive(PartialEq, Eq)]
pub struct Manager {
    entities: Vec<Entity>,
    grouped_entities: HashMap<EntityGroup, Vec<Entity>>,
}

impl Manager {
    pub fn new() -> Manager {
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
            entities_in_group.retain(|e| e.has_group(entity_group) && e.is_active());
        }
        self.entities.retain(|e| e.is_active());
    }

    pub fn add_to_group(&mut self, entity: Entity, group: EntityGroup) {
        match self.grouped_entities.get_mut(&group) {
            Some(g) => {
                g.push(entity);
            }
            None => {
                self.grouped_entities.insert(group, vec![entity]);
            }
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.entities.push(entity);
        entity
    }
}
