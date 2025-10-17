use raylib::prelude::*;

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub vx: f32,
    pub vy: f32,
}

impl Ball {
    pub fn new(x: i32, y: i32, vx: f32, vy: f32) -> Self {
        Ball { x, y, vx, vy }
    }

    pub fn update(&mut self, screen_width: i32, screen_height: i32) {
        self.x += self.vx as i32;
        self.y += self.vy as i32;

        if self.y <= 0 || self.y + 10 >= screen_height {
            self.vy = -self.vy;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x, self.y, 10.0, Color::LIGHTGRAY);
    }
}
