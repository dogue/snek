use crate::render::Vec2;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Vec2>,
}

impl Snake {
    pub fn new(_head_position: Vec2, _body_direction: Vec2) -> Self {
        Self {
            body: vec![Vec2::new(0, 0)],
        }
    }
}
