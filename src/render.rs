/// A two-dimensional vector used for representing positions and directions
#[derive(Debug, Eq, PartialEq)]
pub struct Vec2 {
    x: i32,
    y: i32,
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
