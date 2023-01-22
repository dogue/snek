use crate::utils::Vec2;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum RenderError {
    OutOfBounds(Vec2),
}

impl Error for RenderError {}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::OutOfBounds(pos) => write!(
                f,
                "requested sprite position ({}, {}) would draw outside of window bounds",
                pos.x, pos.y
            ),
        }
    }
}
