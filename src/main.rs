mod paddle;
mod ball;

extern crate sdl2;

use ball::{Ball, Scorer};
use paddle::Paddle;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const FPS_CAP: u32 = 60;
const FRAME_DURATION: u32 = 1_000_000_000;
const FONT_SIZE: i32 = 64;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;
const AI_DISTANCE: i32 = 180;
const LINE_SPACING: i32 = 16;

// SDL Util because I don't want unsafe other places
fn sdl_get_ticks() -> u32 {
    unsafe {
        sdl2::sys::SDL_GetTicks()
    }
}

fn ai_move_paddle(paddle: &mut Paddle, ball: &Ball) {
    let middle = paddle.pos_bounds.y + (paddle.pos_bounds.height() / 2) as i32;

    if ball.pos_bounds.y > middle + AI_DISTANCE {
        paddle.move_offset(1);
    } else if ball.pos_bounds.y < middle - AI_DISTANCE {
        paddle.move_offset(-1);
    }
}

fn draw_net_line(canvas: &mut Canvas<Window>) {
    let mut line_y: i32 = 0;
    while line_y < HEIGHT as i32 {
        canvas.draw_line(
            Point::new((WIDTH / 2) as i32, line_y),
            Point::new((WIDTH / 2) as i32, line_y + LINE_SPACING),
        ).unwrap();
        line_y += LINE_SPACING * 2;
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().expect("TTF could not initialise");
    
    let window = video_subsystem.window("Rust Pong", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // NOTE: To switch between VSync and a fixed/target FPS, I need to recreate the canvas
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let font = ttf_context.load_font("assets/Hack-Regular.ttf", FONT_SIZE as u16).unwrap();
    let mut player_score_tex = font
        .render("0")
        .blended(Color::WHITE)
        .unwrap()
        .as_texture(&texture_creator)
        .unwrap();

    let mut ai_score_tex = font
        .render("0")
        .blended(Color::WHITE)
        .unwrap()
        .as_texture(&texture_creator)
        .unwrap();

    // Delta time
    let mut last_update = sdl_get_ticks();

    let mut player_paddle = Paddle::new(32, 100);
    let mut ai_paddle = Paddle::new((WIDTH - 32) as i32, 150);

    let mut ball = Ball::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32);
    ball.move_offset(-1, -1);

    let mut player_score = 0;
    let mut ai_score = 0;

    'main_loop: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // To draw the paddles and ball, and it's easier to do it here rather than in each draw
        canvas.set_draw_color(Color::WHITE);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop
                },

                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    player_paddle.move_offset(-1);
                }
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    player_paddle.move_offset(1);
                }

                // FIXME: Stop sticky movement, when quickly swapping direction
                Event::KeyUp { keycode: Some(Keycode::W), .. } |
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    player_paddle.move_offset(0);
                }
                _ => {}
            }
        }

        let current_time = sdl_get_ticks();
        // Delta time in seconds
        let delta_time = (current_time - last_update) as f32 / 1000.0f32;
        last_update = current_time;

        // Tick here
        player_paddle.tick(delta_time, HEIGHT as i32);
        ai_move_paddle(&mut ai_paddle, &ball);
        ai_paddle.tick(delta_time, HEIGHT as i32);

        // Dumb structuring but it's fine :^)
        match ball.tick(delta_time, (WIDTH as i32, HEIGHT as i32)) {
            Scorer::Ai => {
                ai_score += 1;

                ai_score_tex = font
                    .render(format!("{}", ai_score).as_str())
                    .blended(Color::WHITE)
                    .unwrap()
                    .as_texture(&texture_creator)
                    .unwrap();
            },
            Scorer::Player => {
                player_score += 1;

                player_score_tex = font
                    .render(format!("{}", player_score).as_str())
                    .blended(Color::WHITE)
                    .unwrap()
                    .as_texture(&texture_creator)
                    .unwrap();
            },
            _ => {}
        }

        ball.check_collision(&player_paddle);
        ball.check_collision(&ai_paddle);

        // Render here
        player_paddle.render(&mut canvas);
        ai_paddle.render(&mut canvas);
        ball.render(&mut canvas);

        draw_net_line(&mut canvas);

        // -- Score
        canvas.copy(&player_score_tex, None, Some(Rect::new((WIDTH / 2) as i32 - (FONT_SIZE * 2) - (FONT_SIZE / 2), 32, 48, 64))).unwrap();
        canvas.copy(&ai_score_tex, None, Some(Rect::new((WIDTH / 2) as i32 + (FONT_SIZE * 2) - (FONT_SIZE / 2), 32, 48, 64))).unwrap();
        
        canvas.present();
        std::thread::sleep(Duration::new(0, FRAME_DURATION / FPS_CAP));
    }
}