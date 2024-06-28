pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

pub struct Vector2DF {
    pub x: f32,
    pub y: f32,
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

impl Zero for Vector2DF {
    fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}
