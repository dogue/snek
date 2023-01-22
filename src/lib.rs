use anyhow::Result;
use color::Color;
use input::get_input;
use minifb::{Window, WindowOptions};
use std::time::Duration;
use world::World;

pub mod color;
pub mod error;
pub mod input;
pub mod render;
pub mod snake;
pub mod text;
pub mod utils;
pub mod world;

pub fn init_window(title: &str, width: usize, height: usize, update_rate: u64) -> Result<Window> {
    let options = WindowOptions::default();

    let mut window = Window::new(title, width, height, options)?;
    window.limit_update_rate(Some(Duration::from_millis(update_rate)));
    window.set_background_color(18, 18, 18);

    Ok(window)
}

pub fn do_tick(mut world: &mut World) -> Result<()> {
    // blank the buffer
    blank(&mut world.buffer);

    // create text object for frametime display
    let time_text = text::Text::new(640, 1, Color::WHITE);

    // get input
    world.input = get_input(&mut world.window);

    // if there is input, move the player sprite, then draw it
    if world.input.is_some() {
        world.player.translate(world.input.unwrap());
    }
    world.player.draw(&mut world.buffer);

    // if running in debug, draw frametime display in lower left corner
    #[cfg(debug_assertions)]
    time_text.draw(
        &mut world.buffer,
        (0, 632),
        &format!("Frame time: {}ms", world.tick_time.elapsed().as_millis()),
    );

    // update window contents with the buffer
    draw(&mut world.window, &mut world.buffer)?;

    // record current moment to calculate frametime on next frame
    world.tick_time = std::time::Instant::now();

    Ok(())
}

fn draw(window: &mut Window, buffer: &Vec<u32>) -> Result<()> {
    window.update_with_buffer(buffer, 640, 640)?;

    Ok(())
}

fn blank(buffer: &mut Vec<u32>) {
    for i in buffer {
        *i = Color::BLANK.into();
    }
}
