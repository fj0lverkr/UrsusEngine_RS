use std::time::Duration;

use constants::{FPS, WINDOW_INIT_H, WINDOW_INIT_W, WINDOW_TITLE};
use game::Game;

extern crate sdl2;

mod camera_2d;
mod constants;
mod ecs;
mod event_system;
mod game;
mod renderer;
mod vector_2d;

fn main() -> Result<(), String> {
    let mut game_context = Game::new(WINDOW_TITLE, WINDOW_INIT_W, WINDOW_INIT_H, false, false)?;
    while game_context.is_running() {
        game_context.update()?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
    Ok(())
}
