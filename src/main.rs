use std::time::Duration;

use game::Game;

extern crate sdl2;

pub mod ecs;
pub mod event_system;
pub mod game;
pub mod renderer;

fn main() -> Result<(), String> {
    let mut game_context = Game::new("Ursus Engine", 1920, 1080, false, false)?;
    while game_context.is_running() {
        game_context.update()?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
