//! Functionality pertaining to a cell in the maze.

use core::fmt::{Debug, Formatter, Result};

pub enum Cell {
    /// Traversable terrain
    PATH,
    /// Impassable terrain
    WALL,
}

impl Debug for Cell {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match *self {
            Self::PATH => {
                write!(
                    formatter,
                    "{}",
                    char::from_u32(0x2B1B).unwrap_or('\u{fffd}')
                )
            }
            Self::WALL => {
                write!(
                    formatter,
                    "{}",
                    char::from_u32(0x2B1C).unwrap_or('\u{fffd}')
                )
            }
        }
    }
}
