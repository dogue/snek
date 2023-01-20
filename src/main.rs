use anyhow::Result;
use snek::{
    color::Color,
    sprite::{Sprite, Vec2},
    GameState,
};

// square window
const WINDOW_SIZE: usize = 640;

fn main() -> Result<()> {
    // create buffer
    let mut buffer: Vec<u32> = vec![Color::BLANK.into(); WINDOW_SIZE * WINDOW_SIZE];
    buffer.fill(0xFF121212);

    // create sprite
    let mut player = Sprite::new(Vec2::new(320, 320));

    // create window
    let mut window = snek::init_window("Framebuffer Snake", WINDOW_SIZE, WINDOW_SIZE, 500)?;

    let mut state = GameState { score: 0 };

    // loop and draw
    while window.is_open() {
        snek::do_tick(&mut window, &mut buffer, &mut player, &mut state)?;
    }

    Ok(())
}
