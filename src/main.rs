use anyhow::Result;

const GAME_WIDTH: usize = 640;
const GAME_HEIGHT: usize = 640;

fn main() -> Result<()> {
    let mut buffer: Vec<u32> = vec![0; GAME_HEIGHT * GAME_WIDTH];

    let mut window = fbsnake::init_window("Framebuffer Snake", GAME_WIDTH, GAME_HEIGHT, 100)?;

    while window.is_open() {
        fbsnake::do_tick(&mut window, &mut buffer)?;
    }

    Ok(())
}
