use anyhow::Result;
use snek::{color::Color, snake::Snake, utils::Vec2, world::World};

// square window
const WINDOW_SIZE: usize = 640;

fn main() -> Result<()> {
    // create buffer and fill with blanking color
    let buffer: Vec<u32> = vec![Color::BLANK.into(); WINDOW_SIZE * WINDOW_SIZE];

    // create snake
    let player = Snake::new(Vec2::new(5, 5));

    // create window
    let window = snek::init_window("Framebuffer Snake", WINDOW_SIZE, WINDOW_SIZE, 1)?;

    // create world object
    let mut world = World {
        score: 0,
        tick_time: std::time::Instant::now(),
        buffer,
        input: None,
        player,
        window,
    };

    // loop and draw
    while world.window.is_open() {
        snek::do_tick(&mut world)?;
    }

    Ok(())
}
