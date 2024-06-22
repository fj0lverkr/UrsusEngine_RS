use sdl2::rect::FRect;

use crate::vector_2d::Vector2DF;

pub struct Camera2D {
    viewfinder: FRect,
}

impl Camera2D {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Camera2D {
        let viewfinder = FRect::new(x, y, width, height);
        Camera2D { viewfinder }
    }

    pub fn viewfinder(self) -> FRect {
        self.viewfinder
    }

    pub fn follow(&mut self, position: &Vector2DF, window_height: f32, window_width: f32) {
        self.viewfinder.x = position.x - window_width / 2f32;
        self.viewfinder.y = position.y - window_height / 2f32;

        // keep viewfinder within window bounds
        if self.viewfinder.x < 0f32 {
            self.viewfinder.x = 0f32;
        }
        if self.viewfinder.y < 0f32 {
            self.viewfinder.y = 0f32;
        }
        if self.viewfinder.x > self.viewfinder.width() {
            self.viewfinder.x = self.viewfinder.width();
        }
        if self.viewfinder.y > self.viewfinder.height() {
            self.viewfinder.y = self.viewfinder.height();
        }
    }
}
