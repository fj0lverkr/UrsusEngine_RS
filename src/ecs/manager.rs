use super::entity::Entity;

const MAXGROUPS: usize = 32;

#[derive(Eq, PartialEq, Hash)]
pub enum EntityLabel {
    Player,
}

#[derive(PartialEq, Eq)]
pub struct Manager {
    entities: Vec<Entity>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
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
        //       for (entity_group, entities_in_group) in &mut self.grouped_entities {
        //          entities_in_group.retain(|e| e.has_group(entity_group) && e.is_active());
        //     }
        self.entities.retain(|e| e.is_active);
    }

    pub fn add_label(&mut self, entity: &Entity, label: EntityLabel) {
        // First check if the given Enity is managed by this Manager
        if let Some(i) = self.entities.iter().position(|e| e == entity) {
            let entity_to_label = self.entities.get_mut(i).unwrap();
            entity_to_label.add_label(label);
        }
    }

    pub fn add_entity(&mut self) -> &Entity {
        let entity = Entity::new();
        self.entities.push(entity);
        self.entities.last().unwrap()
    }

    pub fn add_entity_mut(&mut self) -> &mut Entity {
        let entity = Entity::new();
        self.entities.push(entity);
        self.entities.last_mut().unwrap()
    }
}
