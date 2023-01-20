use anyhow::Result;
use minifb::{Window, WindowOptions};
use sprite::Sprite;
use std::time::Duration;

pub mod error;
pub mod input;
pub mod render;
pub mod snake;
pub mod sprite;
pub mod text;

pub fn init_window(title: &str, width: usize, height: usize, update_rate: u64) -> Result<Window> {
    let options = WindowOptions::default();

    let mut window = Window::new(title, width, height, options)?;
    window.limit_update_rate(Some(Duration::from_millis(update_rate)));
    window.set_background_color(18, 18, 18);

    Ok(window)
}

pub fn do_tick(
    mut window: &mut Window,
    mut buffer: &mut Vec<u32>,
    player: &mut Sprite,
) -> Result<()> {
    // blank the buffer
    blank(&mut buffer);

    // get input
    let input = input::get_input(&mut window);

    // update sprite
    player.update(&mut buffer, input)?;

    // draw
    draw(&mut window, &mut buffer)?;

    Ok(())
}

fn draw(window: &mut Window, buffer: &Vec<u32>) -> Result<()> {
    window.update_with_buffer(buffer, 640, 640)?;

    Ok(())
}

fn blank(buffer: &mut Vec<u32>) {
    for i in buffer {
        *i = 0xFF121212 as u32;
    }
}
