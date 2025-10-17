mod ball;
mod paddle;
mod game_modes;

use raylib::color::Color;
use raylib::prelude::*;
use raylib::consts::KeyboardKey;

use ball::Ball;
use paddle::Paddle;
use game_modes::*;

const SCREEN_WIDTH: i32 = 1200;  // Increased width for better layout
const SCREEN_HEIGHT: i32 = 800;

// Improved color scheme
const BACKGROUND_COLOR: Color = Color::new(10, 20, 30, 255);     // Dark blue
const PRIMARY_COLOR: Color = Color::new(86, 156, 214, 255);      // Bright blue
const SECONDARY_COLOR: Color = Color::new(220, 120, 70, 255);    // Orange
const ACCENT_COLOR: Color = Color::new(120, 220, 120, 255);      // Green
const TEXT_COLOR: Color = Color::new(240, 240, 240, 255);        // Off-white
const BALL_COLOR: Color = Color::new(255, 215, 0, 255);          // Gold

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ping-Pong Enhanced")
        .build();
    rl.set_target_fps(60);

    // Load custom font from assets
    let custom_font = match rl.load_font(&thread, "assets/NotoSans-Regular.ttf") {
        Ok(font) => font,
        Err(_) => {
            println!("Failed to load custom font, using default");
            rl.get_font_default()
        }
    };

    let mut ball = Ball::init(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 5.0, 5.0);
    let mut paddle1 = Paddle::init(80, (SCREEN_HEIGHT / 2) - 50, 5.0); // Moved left
    let mut paddle2 = Paddle::init(SCREEN_WIDTH - 80 - 15, (SCREEN_HEIGHT / 2) - 50, 5.0); // Adjusted position

    let mut score1 = 0;
    let mut score2 = 0;
    let mut timer = 60.0;
    let mut game_mode = 0;

    while !rl.window_should_close() {
        if game_mode == 0 {
            // Main menu
            if let Some(key) = rl.get_key_pressed() {
                match key {
                    KeyboardKey::KEY_ONE => {
                        game_mode = 1;
                        reset_game(&mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer);
                    },
                    KeyboardKey::KEY_TWO => {
                        game_mode = 2;
                        reset_game(&mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer);
                    },
                    _ => {}
                }
            }

            let mut draw = rl.begin_drawing(&thread);
            draw.clear_background(BACKGROUND_COLOR);
            
            // Improved menu text with better positioning and colors
            draw.draw_text_ex(
                &custom_font, 
                "PING-PONG", 
                Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 120.0, 100.0), 
                60.0, 
                0.0, 
                PRIMARY_COLOR
            );
            
            draw.draw_text_ex(
                &custom_font,
                "Press 1 for Multi-Player",
                Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 180.0, SCREEN_HEIGHT as f32 / 2.0 - 40.0),
                32.0,
                0.0,
                SECONDARY_COLOR
            );
            
            draw.draw_text_ex(
                &custom_font,
                "Press 2 for Single Player", 
                Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 180.0, SCREEN_HEIGHT as f32 / 2.0 + 20.0),
                32.0,
                0.0,
                ACCENT_COLOR
            );
        } else {
            match game_mode {
                1 => multi_player(&mut rl, &thread, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &custom_font),
                2 => single_player(&mut rl, &thread, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &custom_font),
                _ => unreachable!(),
            }
            if timer <= 0.0 {
                break;
            }
        }
    }

    // Winner screen 
    let mut draw = rl.begin_drawing(&thread);
    draw.clear_background(BACKGROUND_COLOR);
    
    let winner_text = if score1 > score2 {
        "Player 1 Wins!"
    } else if score2 > score1 {
        "Player 2 Wins! "
    } else {
        "It's a Tie! "
    };
    
    let final_score = format!("Final Score: {} - {}", score1, score2);
    
    draw.draw_text_ex(
        &custom_font,
        winner_text,
        Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 200.0, SCREEN_HEIGHT as f32 / 2.0 - 60.0),
        48.0,
        0.0,
        PRIMARY_COLOR
    );
    
    draw.draw_text_ex(
        &custom_font,
        &final_score,
        Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 120.0, SCREEN_HEIGHT as f32 / 2.0 + 20.0),
        36.0,
        0.0,
        TEXT_COLOR
    );
    
    draw.draw_text_ex(
        &custom_font,
        "Press ESC to exit",
        Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 120.0, SCREEN_HEIGHT as f32 / 2.0 + 80.0),
        24.0,
        0.0,
        SECONDARY_COLOR
    );
    
    drop(draw);

    while !rl.window_should_close() {
        rl.begin_drawing(&thread);
    }
}

fn reset_game(ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32) {
    *ball = Ball::init(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 5.0, 5.0);
    *paddle1 = Paddle::init(80, (SCREEN_HEIGHT / 2) - 50, 5.0);  // Adjusted position
    *paddle2 = Paddle::init(SCREEN_WIDTH - 80 - 15, (SCREEN_HEIGHT / 2) - 50, 5.0); // Adjusted position
    *score1 = 0;
    *score2 = 0;
    *timer = 60.0;
}