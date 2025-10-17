use raylib::prelude::*;

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub speed: f32,
    pub width: i32,
    pub height: i32,
}

impl Paddle {
    pub fn init(x: i32, y: i32, speed: f32) -> Self {
        Paddle {
            x, 
            y, 
            speed,
            width: 15,  // Increased width for better visibility
            height: 120, // Slightly taller for better gameplay
        }
    }

    pub fn update(&mut self, screen_height: i32, up: bool, down: bool) {
        if up {
            self.y -= self.speed as i32;  // sign for changing direction
        }
        if down {
            self.y += self.speed as i32;
        }
        if self.y < 0 {
            self.y = 0;
        }
        if self.y + self.height > screen_height {
            self.y = screen_height - self.height;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, is_player1: bool) {
        let color = if is_player1 {
            // Player 1 (left paddle) - Blue theme
            Color::new(86, 156, 214, 255)
        } else {
            // Player 2 (right paddle) - Orange theme  
            Color::new(220, 120, 70, 255)
        };
        
        let outline_color = if is_player1 {
            Color::new(60, 130, 200, 255)
        } else {
            Color::new(200, 90, 50, 255)
        };

        // Draw paddle with gradient effect
        d.draw_rectangle_gradient_v(
            self.x, 
            self.y, 
            self.width, 
            self.height, 
            color, 
            Color::new(color.r - 30, color.g - 30, color.b - 30, 255)
        );
        
        // outline for better visibility
        d.draw_rectangle_lines(self.x, self.y, self.width, self.height, outline_color);
        
        // subtle inner highlight
        d.draw_rectangle_lines(self.x + 2, self.y + 2, self.width - 4, self.height - 4, Color::new(255, 255, 255, 80));
    }
    
    // Helper method for collision detection
    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32
        )
    }
}