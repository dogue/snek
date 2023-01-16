use anyhow::Result;
use minifb::{Key, Window};

pub fn get_input(window: &mut Window) -> Result<()> {
    window.get_keys().iter().for_each(|key| match key {
        Key::Escape => std::process::exit(0),
        _ => {}
    });

    Ok(())
}
