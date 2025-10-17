use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use raylib::color::Color;
use crate::ball::Ball;
use crate::paddle::Paddle;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

pub fn single_player(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    ball: &mut Ball,
    paddle1: &mut Paddle,
    paddle2: &mut Paddle,
    score1: &mut i32,
    score2: &mut i32,
    timer: &mut f32,
    font: &WeakFont,
    ai_difficulty: i32
) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::BLACK);

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    let ai_speed = match ai_difficulty {
        1 => 3.0, // easy
        2 => 4.5, // medium
        3 => 6.0, // hard
        _ => 3.0,
    };
    let target_y = (ball.y - 50).clamp(0, SCREEN_HEIGHT - 100);
    if (paddle1.y - target_y).abs() > ai_speed as i32 {
        if paddle1.y < target_y {
            paddle1.y += ai_speed as i32;
        } else {
            paddle1.y -= ai_speed as i32;
        }
    } else {
        paddle1.y = target_y;
    }

    update_game_state(ball, paddle1, paddle2, score1, score2);
    *timer -= draw.get_frame_time();
    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

pub fn multi_player(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    ball: &mut Ball,
    paddle1: &mut Paddle,
    paddle2: &mut Paddle,
    score1: &mut i32,
    score2: &mut i32,
    timer: &mut f32,
    font: &WeakFont,
    cooldown: &mut f32,
    frame_time: f32
) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::BLACK);

    if *cooldown > 0.0 {
        *cooldown -= frame_time;
        draw.draw_text(
            &format!("Next round in {:.1}...", cooldown.max(0.0)),
            SCREEN_WIDTH/2 - 100, SCREEN_HEIGHT/2, 30, Color::WHITE
        );
        draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
        return;
    }

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle1.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_W), draw.is_key_down(KeyboardKey::KEY_S));
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    let mut scored = false;
    if (ball.x + 10) >= SCREEN_WIDTH {
        *score1 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
        *cooldown = 2.0;
        scored = true;
    }
    if ball.x <= 0 {
        *score2 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
        *cooldown = 2.0;
        scored = true;
    }

    if !scored {
        update_game_state(ball, paddle1, paddle2, score1, score2);
        *timer -= frame_time;
    }

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

fn update_game_state(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: &mut i32, score2: &mut i32){
    if ball.vx < 0.0 && (ball.x <= paddle1.x + 10) && (ball.x + 10 >= paddle1.x) && (ball.y + 10 >= paddle1.y) && (ball.y <= paddle1.y + 100) {
        ball.x = paddle1.x + 10; 
        ball.vx = -ball.vx;
    }

    if ball.vx > 0.0 && (ball.x + 10 >= paddle2.x) && (ball.x <= paddle2.x + 10) && (ball.y + 10 >= paddle2.y) && (ball.y <= paddle2.y + 100) {
        ball.x = paddle2.x - 10; 
        ball.vx = -ball.vx;
    }
    if (ball.x + 10) >= SCREEN_WIDTH{
        *score1 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
    }
    if ball.x <= 0{
        *score2 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: i32, score2: i32, timer: f32, font: &WeakFont){
    draw.clear_background(Color::new(24, 29, 49, 255));
    draw.draw_text_ex(font, &format!("Player 1: {}", score1), Vector2::new(24.0, 36.0), 38.0, 2.0, Color::new(230, 221, 59, 255));
    draw.draw_text_ex(font, &format!("Player 2: {}", score2), Vector2::new(24.0, 86.0), 38.0, 2.0, Color::new(127, 199, 217, 255));
    draw.draw_text_ex(font, &format!("Timer: {:>4.1}", timer), Vector2::new(24.0, 136.0), 32.0, 2.0, Color::new(247, 247, 247, 255));

    paddle1.draw(draw);
    paddle2.draw(draw);
    ball.draw(draw);
}
