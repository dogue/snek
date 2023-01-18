use crate::sprite::Direction;
use anyhow::{Ok, Result};
use minifb::{Key, KeyRepeat, Window};

pub fn get_input(window: &mut Window) -> Result<Option<Direction>> {
    if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        std::process::exit(0);
    }

    for key in window.get_keys().iter() {
        match key {
            Key::A => return Ok(Some(Direction::Left)),
            Key::D => return Ok(Some(Direction::Right)),
            Key::W => return Ok(Some(Direction::Up)),
            Key::S => return Ok(Some(Direction::Down)),
            _ => return Ok(None),
        }
    }

    Ok(None)
}
