use raylib::prelude::*;
use crate::ball::Ball;
use crate::paddle::Paddle;
use rand::Rng;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

fn random_velocity() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let vx = if rng.gen_bool(0.5) { 5.0 } else { -5.0 };
    let vy = if rng.gen_bool(0.5) { 5.0 } else { -5.0 };
    (vx, vy)
}

pub fn single_player(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    ball: &mut Ball,
    paddle1: &mut Paddle, // AI
    paddle2: &mut Paddle, // Player
    score1: &mut i32,
    score2: &mut i32,
    timer: &mut f32,
    font: &Font,
) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::BLACK);

    ball.update(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Player paddle
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    // AI paddle
    let target_y = (ball.y - 50).clamp(0, SCREEN_HEIGHT - 100);
    if (paddle1.y - target_y).abs() > paddle1.speed as i32 {
        if paddle1.y < target_y { paddle1.y += paddle1.speed as i32; }
        else { paddle1.y -= paddle1.speed as i32; }
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
    font: &Font,
) {
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::BLACK);

    ball.update(SCREEN_WIDTH, SCREEN_HEIGHT);

    paddle1.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_W), draw.is_key_down(KeyboardKey::KEY_S));
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    update_game_state(ball, paddle1, paddle2, score1, score2);

    *timer -= draw.get_frame_time();

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

fn update_game_state(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: &mut i32, score2: &mut i32) {
    // Ball collisions
    if ball.x <= paddle1.x + 10 && ball.y >= paddle1.y && ball.y <= paddle1.y + 100 {
        ball.vx = -ball.vx;
    }
    if ball.x + 10 >= paddle2.x && ball.y >= paddle2.y && ball.y <= paddle2.y + 100 {
        ball.vx = -ball.vx;
    }

    // Score
    if ball.x + 10 >= SCREEN_WIDTH {
        *score1 += 1;
        let (vx, vy) = random_velocity();
        *ball = Ball::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, vx, vy);
    }
    if ball.x <= 0 {
        *score2 += 1;
        let (vx, vy) = random_velocity();
        *ball = Ball::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, vx, vy);
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, ball: &Ball, paddle1: &Paddle, paddle2: &Paddle, score1: i32, score2: i32, timer: f32, font: &Font) {
    draw.clear_background(Color::BLACK);

    draw.draw_text_ex(font, &format!("Player 1: {}", score1), Vector2::new(20.0, 10.0), 40.0, 2.0, Color::AQUA);
    draw.draw_text_ex(font, &format!("Timer: {}", timer.floor() as i32), Vector2::new((SCREEN_WIDTH / 2 - 70) as f32, 10.0), 40.0, 2.0, Color::AQUA);
    draw.draw_text_ex(font, &format!("Player 2: {}", score2), Vector2::new((SCREEN_WIDTH - 180) as f32, 10.0), 40.0, 2.0, Color::AQUA);

    paddle1.draw(draw);
    paddle2.draw(draw);
    ball.draw(draw);
}
