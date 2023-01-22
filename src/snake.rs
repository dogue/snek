use crate::{
    render::draw_rect,
    utils::{Direction, Vec2},
};

#[derive(Debug)]
pub struct Snake {
    head: Vec2,
    body: Vec<Vec2>,
}

impl Snake {
    pub fn new(position: Vec2) -> Self {
        Self {
            head: position,
            body: vec![],
        }
    }

    pub fn draw(&self, mut buffer: &mut Vec<u32>) {
        draw_rect(self.head, &mut buffer).unwrap();
    }

    pub fn translate(&mut self, direction: Direction) {
        let delta = match direction {
            Direction::Up => Vec2::UP,
            Direction::Down => Vec2::DOWN,
            Direction::Left => Vec2::LEFT,
            Direction::Right => Vec2::RIGHT,
        } * 32;

        self.head = self.head + delta;
    }
}
