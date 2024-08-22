//! Functionality pertaining to a tile in the maze.

use core::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone)]
pub enum Tile {
    /// Impassable terrain
    Wall,
    /// Traversable terrain
    Path,
}

impl Display for Tile {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match *self {
            Self::Path => {
                write!(
                    formatter,
                    "{}",
                    char::from_u32(0x2B1C).unwrap_or('\u{fffd}')
                )
            }
            Self::Wall => {
                write!(
                    formatter,
                    "{}",
                    char::from_u32(0x2B1B).unwrap_or('\u{fffd}')
                )
            }
        }
    }
}

impl Debug for Tile {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{self}")
    }
}

impl Default for Tile {
    #[inline]
    fn default() -> Self {
        Self::Wall
    }
}
