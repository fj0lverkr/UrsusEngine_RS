use sdl2::rect::Rect;

use crate::vector_2d::Vector2D;

pub struct Camera2D {
    viewfinder: Rect,
}

impl Camera2D {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Camera2D {
        let viewfinder = Rect::new(x, y, width, height);
        Camera2D { viewfinder }
    }

    pub fn viewfinder(self) -> Rect {
        self.viewfinder
    }

    pub fn follow(&mut self, position: &Vector2D, window_height: f32, window_width: f32) {
        self.viewfinder.x = position.x - (window_width / 2f32) as i32;
        self.viewfinder.y = position.y - (window_height / 2f32) as i32;

        // keep viewfinder within window bounds
        if self.viewfinder.x < 0 {
            self.viewfinder.x = 0;
        }
        if self.viewfinder.y < 0 {
            self.viewfinder.y = 0;
        }
        if self.viewfinder.x > self.viewfinder.width() as i32 {
            self.viewfinder.x = self.viewfinder.width() as i32;
        }
        if self.viewfinder.y > self.viewfinder.height() as i32 {
            self.viewfinder.y = self.viewfinder.height() as i32;
        }
    }
}
