use crate::render;

/// Represents a single square sprite
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    // color:
}

impl Sprite {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub fn pos(&self) -> Vec2 {
        self.position
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn translate(&mut self, direction: Direction) {
        let new_pos = match direction {
            Direction::Left => Vec2::new(self.position.x - 32, self.position.y),
            Direction::Right => Vec2::new(self.position.x + 32, self.position.y),
            Direction::Up => Vec2::new(self.position.x, self.position.y - 32),
            Direction::Down => Vec2::new(self.position.x, self.position.y + 32),
        };

        if !render::out_of_bounds(&new_pos) {
            self.position = new_pos;
        }
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// A two-dimensional vector used for representing positions and directions
#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    /// 0, -1
    pub const UP: Vec2 = Vec2 { x: 0, y: -1 };

    /// 0, 1
    pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };

    /// -1, 0
    pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };

    /// 1, 0
    pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

    /// 0, 0
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
