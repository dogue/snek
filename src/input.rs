use crate::utils::Direction;
use minifb::{Key, KeyRepeat, Window};

pub fn get_input(window: &mut Window) -> Option<Direction> {
    if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        std::process::exit(0);
    }

    for key in window.get_keys_pressed(KeyRepeat::No).iter() {
        match key {
            Key::A => return Some(Direction::Left),
            Key::D => return Some(Direction::Right),
            Key::W => return Some(Direction::Up),
            Key::S => return Some(Direction::Down),
            _ => return None,
        }
    }

    None
}
