mod game;

use sdl2::event::Event;
use std::time::Duration;

const WINDOW_WIDTH: usize = 1600;
const WINDOW_HEIGHT: usize = 900;
const PIXEL_SIZE: usize = 20;
const WINDOW_NAME: &str = "PADDLE";
const FONT_PATH: &str = "fonts/monogram.ttf";
const FONT_SIZE: u16 = 50;
const FPS: usize = 30;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window(WINDOW_NAME, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font(FONT_PATH, FONT_SIZE)?;

    let mut game = game::Game::new(
        WINDOW_WIDTH / PIXEL_SIZE,
        WINDOW_HEIGHT / PIXEL_SIZE,
        PIXEL_SIZE,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    game.dispatch_key(keycode, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    game.dispatch_key(keycode, false);
                }
                _ => {}
            }
        }
        game.update();
        game.draw(&mut canvas, &texture_creator, &font);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS as u32));
    }
    Ok(())
}
