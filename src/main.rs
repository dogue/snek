use anyhow::Result;
use fbsnake::sprite::{Sprite, Vec2};

// square window
const WINDOW_SIZE: usize = 640;

fn main() -> Result<()> {
    // create buffer
    let mut buffer: Vec<u32> = vec![0xFF121212; WINDOW_SIZE * WINDOW_SIZE];
    buffer.fill(0xFF121212);

    // create sprite
    let mut player = Sprite::new(Vec2::new(0, 0));

    // create window
    let mut window = fbsnake::init_window("Framebuffer Snake", WINDOW_SIZE, WINDOW_SIZE, 1)?;

    // loop and draw
    while window.is_open() {
        fbsnake::do_tick(&mut window, &mut buffer, &mut player)?;
    }

    Ok(())
}
