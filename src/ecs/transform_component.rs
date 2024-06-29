use crate::vector_2d::{Vector2D, Zero};

use super::component::{BaseComponent, ComponentBehavior};

#[derive(Eq, PartialEq)]
pub struct TransformComponent {
    pub component: BaseComponent,
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub width: i32,
    pub height: i32,
    pub scale: u32,
    pub speed: i32,
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        let component = BaseComponent::new();
        let position = Vector2D { x: 0, y: 0 };
        let velocity = Vector2D { x: 0, y: 0 };
        let width = 32;
        let height = 32;
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
    pub fn new_with_pos(x: i32, y: i32) -> TransformComponent {
        let component = BaseComponent::new();
        let position = Vector2D { x, y };
        let velocity = Vector2D { x: 0, y: 0 };
        let width = 32;
        let height = 32;
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
        self.position.x += self.velocity.x * self.speed;
        self.position.y += self.velocity.y * self.speed;
    }
}
