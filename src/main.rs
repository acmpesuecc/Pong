mod ball;
mod paddle;
mod game_modes;

use::raylib::color::Color;
use::raylib::prelude::*;
use::raylib::consts::KeyboardKey;

use ball::Ball;
use paddle::Paddle;
use game_modes::*;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

fn main(){
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ping-Pong")
        .build();
    rl.set_target_fps(60);

    let mut custom_font = rl.get_font_default();

    let mut ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT, 5.0, 5.0);
    let mut paddle1 = Paddle::init(50, (SCREEN_HEIGHT/2)-50, 5.0); 
    let mut paddle2 = Paddle::init(SCREEN_WIDTH-50-10, (SCREEN_HEIGHT/2)-50, 5.0);

    let mut score1 = 0;
    let mut score2 = 0;
    let mut timer = 60.0; 

    let mut game_mode = 0; 
    let mut cooldown = 0.0; 

    while !rl.window_should_close() {
        if game_mode == 0{
            if let Some(key) =  rl.get_key_pressed(){
                match key{
                    KeyboardKey::KEY_ONE => {
                        game_mode = 1;
                        reset_game(&mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer);
                        cooldown = 0.0;
                    },
                    KeyboardKey::KEY_TWO => {
                        game_mode = 2;
                        reset_game(&mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer);
                        cooldown = 0.0;
                    },
                    _ => {}
                }
            }

            let mut draw = rl.begin_drawing(&thread);
            draw.clear_background(Color::BLACK);
            draw.draw_text("Press 1 for multi-player", SCREEN_WIDTH/2 - 150, SCREEN_HEIGHT/2 - 30, 30, Color::WHITE);
            draw.draw_text("Press 2 for single player", SCREEN_WIDTH/2 - 150, SCREEN_HEIGHT/2 + 30, 30, Color::WHITE);
        }
        else {
            let frame_time = rl.get_frame_time();
            match game_mode {
                1 => multi_player(&mut rl,  &thread, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &custom_font, &mut cooldown, frame_time),
                2 => single_player(&mut rl, &thread, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &custom_font),
                _ => unreachable!(),
            }
            if timer <= 0.0{
                break;
            }
        }
    }

    let mut draw = rl.begin_drawing(&thread);
    draw.clear_background(Color::BLACK);
    if score1>score2{
        draw.draw_text("Player1 wins!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    else if score2>score1{
        draw.draw_text("Player2 wins!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    else{
        draw.draw_text("Tie!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    draw.draw_text("Press ESC to exit", SCREEN_WIDTH/2 - 100, SCREEN_HEIGHT/2 + 60, 30, Color::WHITE);
    drop(draw);

    while !rl.window_should_close() {
        rl.begin_drawing(&thread);
    }
}

fn reset_game(ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32){
    *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
    *paddle1 = Paddle::init(50, (SCREEN_HEIGHT/2) - 50, 5.0);
    *paddle2 = Paddle::init(SCREEN_WIDTH - 50 - 10, (SCREEN_HEIGHT/2) - 50, 5.0);
    *score1 = 0;
    *score2 = 0;
    *timer = 60.0;
}
