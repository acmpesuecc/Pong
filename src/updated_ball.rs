use raylib::prelude::*;

pub struct Ball {
    pub x: i32,
    pub y: i32, // positions
    pub vx: f32,
    pub vy: f32, // velocities
    pub radius: f32,
}

impl Ball {
    pub fn init(x: i32, y: i32, vx: f32, vy: f32) -> Self {
        Ball {
            x,
            y,
            vx,
            vy,
            radius: 12.0, // Slightly larger for better visibility
        }
    }

    pub fn change_dir(&mut self, screen_width: i32, screen_height: i32) {
        self.x += self.vx as i32;
        self.y += self.vy as i32;

        if self.x <= 0 || self.x + self.radius as i32 >= screen_width {
            self.vx = -self.vx; // change direction after collision
        }

        if self.y <= 0 || self.y + self.radius as i32 >= screen_height {
            self.vy = -self.vy;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        // Improved ball with gradient effect
        d.draw_circle_gradient(
            self.x,
            self.y,
            self.radius,
            Color::new(255, 215, 0, 255),    // Gold center
            Color::new(255, 165, 0, 255),    // Orange edges
        );
        
        //subtle outline
        d.draw_circle_lines(self.x, self.y, self.radius, Color::new(255, 200, 50, 255));
    }
    
    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            self.x as f32 - self.radius,
            self.y as f32 - self.radius,
            self.radius * 2.0,
            self.radius * 2.0
        )
    }
}