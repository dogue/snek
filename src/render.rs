use crate::error::RenderError;
use crate::sprite::Vec2;

pub fn draw_rect(position: Vec2, buffer: &mut Vec<u32>) -> Result<(), RenderError> {
    if out_of_bounds(&position) {
        return Err(RenderError::OutOfBounds(position));
    }

    let origin = (position.y * 640) + position.x;
    let line: [u32; 32] = [0xFFFFFFFF; 32];

    for i in 0..32 {
        let row = (origin + (640 * i as i32)) as usize;
        buffer.splice(row..=(row + 31), line);
    }

    Ok(())
}

pub fn out_of_bounds(position: &Vec2) -> bool {
    let buffer_len = 640 * 640;
    let origin = (position.y * 640) + position.x;

    if origin < 0 || origin >= buffer_len {
        return true;
    }

    if position.x < 0 || position.y < 0 {
        return true;
    }

    // right hand edge exceeds the right most pixel of this row
    if position.x >= 640 {
        return true;
    }

    false
}
