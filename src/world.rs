use std::time::Instant;

use minifb::Window;

use crate::snake::Snake;
use crate::utils::Direction;

#[derive(Debug)]
pub struct World {
    pub score: usize,
    pub tick_time: Instant,
    pub buffer: Vec<u32>,
    pub input: Option<Direction>,
    pub player: Snake,
    pub window: Window,
}
