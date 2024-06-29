#[derive(Eq, PartialEq)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

pub trait Zero {
    fn zero(&mut self);
}

impl Zero for Vector2D {
    fn zero(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}
