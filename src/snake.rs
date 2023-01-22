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
            head: position * 32,
            body: vec![],
        }
    }

    pub fn draw(&self, mut buffer: &mut Vec<u32>) {
        let _ = draw_rect(self.head, &mut buffer);
    }

    pub fn translate(&mut self, direction: Direction) {
        let delta = match direction {
            Direction::Up => Vec2::UP,
            Direction::Down => Vec2::DOWN,
            Direction::Left => Vec2::LEFT,
            Direction::Right => Vec2::RIGHT,
        } * 32;

        self.head = self.head + delta;
        self.head.x = self.head.x.clamp(0, 640 - 32);
        self.head.y = self.head.y.clamp(0, 640 - 32);
        dbg!(self.head);
    }
}
