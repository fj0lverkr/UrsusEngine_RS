use crate::vector_2d::{Vector2DF, Zero};

use super::component::{BaseComponent, ComponentBehavior};

pub struct TransformComponent {
    pub component: BaseComponent,
    pub position: Vector2DF,
    pub velocity: Vector2DF,
    pub width: f32,
    pub height: f32,
    pub scale: u32,
    pub speed: i32,
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        let component = BaseComponent::new();
        let position = Vector2DF { x: 0f32, y: 0f32 };
        let velocity = Vector2DF { x: 0f32, y: 0f32 };
        let width = 32f32;
        let height = 32f32;
        let scale = 1u32;
        let speed = 3;
        TransformComponent {
            component,
            position,
            velocity,
            width,
            height,
            scale,
            speed,
        }
    }
    pub fn new_with_pos(x: f32, y: f32) -> TransformComponent {
        let component = BaseComponent::new();
        let position = Vector2DF { x, y };
        let velocity = Vector2DF { x: 0f32, y: 0f32 };
        let width = 32f32;
        let height = 32f32;
        let scale = 1u32;
        let speed = 3;
        TransformComponent {
            component,
            position,
            velocity,
            width,
            height,
            scale,
            speed,
        }
    }
}

impl ComponentBehavior for TransformComponent {
    fn init(&mut self) {
        self.position.zero();
    }

    fn draw(&self) {}

    fn update(&mut self) {
        self.position.x += self.velocity.x * self.speed as f32;
        self.position.y += self.velocity.y * self.speed as f32;
    }
}
