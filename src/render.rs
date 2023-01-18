use crate::error::RenderError;
use crate::sprite::Vec2;

pub fn draw_rect(position: Vec2, _size: Vec2, buffer: &mut Vec<u32>) -> Result<(), RenderError> {
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
    let origin = (position.y * 640) + position.x;

    if origin < 0 {
        return true;
    }

    let edge_y = origin + 32;
    let edge_x = origin + (32 * 640);
    let buffer_len = 640 * 640;

    // either axis exceeds the end of the buffer
    if edge_x >= buffer_len || edge_y >= buffer_len {
        return true;
    }

    // right hand edge exceeds the right most pixel of this row
    if position.x >= 640 - 32 {
        return true;
    }

    false
}
