use super::entity::Entity;

pub struct Manager<'a> {
    entities: Vec<Box<Entity>>,
    grouped_entities: Vec<Vec<&'a Entity>>,
}

impl<'a> Manager<'_> {
    fn new() -> Manager<'a> {
        Manager {
            entities: Vec::new(),
            grouped_entities: Vec::with_capacity(32),
        }
    }

    fn update(&mut self) {
        for e in &mut self.entities {
            e.update();
        }
    }
}
