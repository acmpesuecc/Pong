use raylib::prelude::*;

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: i32, y: i32, speed: f32) -> Self {
        Paddle { x, y, speed }
    }

    pub fn update(&mut self, screen_height: i32, up: bool, down: bool) {
        if up {
            self.y -= self.speed as i32;
        }
        if down {
            self.y += self.speed as i32;
        }
        if self.y < 0 {
            self.y = 0;
        }
        if self.y + 100 > screen_height {
            self.y = screen_height - 100;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x, self.y, 10, 100, Color::LIGHTBLUE);
    }
}
