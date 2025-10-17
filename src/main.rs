use raylib::prelude::*;
mod paddle;
mod ball;
mod game_modes;

use paddle::Paddle;
use ball::Ball;
use game_modes::{single_player, multi_player};

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

fn main() {
    let (mut rl, thread) = init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Pong Daddy");
    rl.set_target_fps(60);

    let font = rl.load_font(&thread, "resources/arial.ttf").unwrap();

    let mut paddle1 = Paddle::new(50, 350, 5.0);
    let mut paddle2 = Paddle::new(940, 350, 5.0);
    let mut ball = Ball::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 5.0, 5.0);

    let mut score1 = 0;
    let mut score2 = 0;
    let mut timer = 60.0;

    println!("Press 1 for Multiplayer, 2 for Single Player");

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            multi_player(&mut rl, &thread, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &font);
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            single_player(&mut rl, &thread, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &font);
        }

        if timer <= 0.0 {
            println!("Game Over! Player 1: {} Player 2: {}", score1, score2);
            break;
        }
    }
}
