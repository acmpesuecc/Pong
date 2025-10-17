use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use crate::ball::Ball;
use crate::paddle::Paddle;

const SCREEN_WIDTH: i32 = 1200;  // Increased width for better layout
const SCREEN_HEIGHT: i32 = 800;

// Improved color scheme
const BACKGROUND_COLOR: Color = Color::new(10, 20, 30, 255);
const SCORE_COLOR: Color = Color::new(240, 240, 240, 255);
const TIMER_COLOR: Color = Color::new(120, 220, 120, 255);
const CENTER_LINE_COLOR: Color = Color::new(60, 60, 80, 255);

pub fn single_player(rl: &mut RaylibHandle, thread: &RaylibThread, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32, font: &Font) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(BACKGROUND_COLOR);

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    // Auto-controlled paddle
    let target_y = (ball.y - paddle1.height / 2).clamp(0, SCREEN_HEIGHT - paddle1.height);
    if (paddle1.y - target_y).abs() > paddle1.speed as i32 {
        if paddle1.y < target_y {
            paddle1.y += paddle1.speed as i32;
        } else {
            paddle1.y -= paddle1.speed as i32;
        }
    } else {
        paddle1.y = target_y;
    }

    update_game_state(ball, paddle1, paddle2, score1, score2);
    *timer -= draw.get_frame_time();

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

pub fn multi_player(rl: &mut RaylibHandle, thread: &RaylibThread, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32, font: &Font) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(BACKGROUND_COLOR);

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle1.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_W), draw.is_key_down(KeyboardKey::KEY_S));
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    update_game_state(ball, paddle1, paddle2, score1, score2);
    *timer -= draw.get_frame_time();

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

fn update_game_state(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: &mut i32, score2: &mut i32) {
    // Improved collision detection using bounds
    let ball_bounds = ball.get_bounds();
    let paddle1_bounds = paddle1.get_bounds();
    let paddle2_bounds = paddle2.get_bounds();

    if ball_bounds.check_collision_recs(&paddle1_bounds) {
        ball.vx = ball.vx.abs(); // Move right
        // Add some vertical variation based on where the ball hit the paddle
        let hit_pos = (ball.y - paddle1.y) as f32 / paddle1.height as f32 - 0.5;
        ball.vy += hit_pos * 2.0;
    }
    
    if ball_bounds.check_collision_recs(&paddle2_bounds) {
        ball.vx = -ball.vx.abs(); // Move left
        // Add some vertical variation based on where the ball hit the paddle
        let hit_pos = (ball.y - paddle2.y) as f32 / paddle2.height as f32 - 0.5;
        ball.vy += hit_pos * 2.0;
    }

    // Score when ball goes past paddles
    if ball.x <= 0 {
        *score2 += 1;
        *ball = Ball::init(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 5.0, 0.0);
    }
    if ball.x >= SCREEN_WIDTH {
        *score1 += 1;
        *ball = Ball::init(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, -5.0, 0.0);
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: i32, score2: i32, timer: f32, font: &Font) {
    draw.clear_background(BACKGROUND_COLOR);
    
    // Draw center line (dashed)
    for y in (0..SCREEN_HEIGHT).step_by(20) {
        draw.draw_rectangle(SCREEN_WIDTH / 2 - 2, y, 4, 10, CENTER_LINE_COLOR);
    }
    
    // IMPROVED SCOREBOARD - Moved to left side and fully visible
    let score_panel_width = 300; // Dedicated space for scores on the left
    
    // Draw semi-transparent background for score panel
    draw.draw_rectangle(0, 0, score_panel_width, 120, Color::new(20, 30, 40, 220));
    
    // Player 1 score - Left aligned in the panel
    draw.draw_text_ex(
        font,
        &format!("PLAYER 1: {}", score1),
        Vector2::new(20.0, 20.0),
        32.0,
        0.0,
        Color::new(86, 156, 214, 255) // Blue
    );
    
    // Player 2 score - Below player 1
    draw.draw_text_ex(
        font,
        &format!("PLAYER 2: {}", score2),
        Vector2::new(20.0, 60.0),
        32.0,
        0.0,
        Color::new(220, 120, 70, 255) // Orange
    );
    
    // Timer - Centered in the score panel
    let timer_text = format!("TIME: {:.1}", timer);
    let timer_text_size = measure_text_ex(font, &timer_text, 28.0, 0.0);
    draw.draw_text_ex(
        font,
        &timer_text,
        Vector2::new((score_panel_width / 2) as f32 - timer_text_size.x / 2.0, 95.0),
        28.0,
        0.0,
        TIMER_COLOR
    );

    // Draw game objects with improved colors
    paddle1.draw(draw, true);  // true for player 1 (blue)
    paddle2.draw(draw, false); // false for player 2 (orange)
    ball.draw(draw);
    
    // Draw control hints at the bottom
    if SCREEN_WIDTH > 1000 { // Only show if screen is wide enough
        draw.draw_text_ex(
            font,
            "Player 1: W/S  |  Player 2: ↑/↓",
            Vector2::new(SCREEN_WIDTH as f32 / 2.0 - 180.0, SCREEN_HEIGHT as f32 - 40.0),
            20.0,
            0.0,
            Color::new(150, 150, 150, 255)
        );
    }
}