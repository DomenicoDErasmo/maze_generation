//! A pair of co-ordinates to enable easy vector access.

use core::ops::{Add, Sub};

use crate::direction::Direction;

/// Denotes a row-and-column pair to access a 2-D vector.
#[derive(Copy, Clone, Default)]
pub struct Pair {
    /// The row to access from.
    pub row: i32,
    /// The column to access from.
    pub col: i32,
}

impl Pair {
    /// Creates a Pair from a row and col.
    #[inline]
    #[must_use]
    pub const fn from_row_and_col(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl From<Direction> for Pair {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Down => Self::from_row_and_col(-2, 0),
            Direction::Left => Self::from_row_and_col(0, -2),
            Direction::Up => Self::from_row_and_col(2, 0),
            Direction::Right => Self::from_row_and_col(0, 2),
        }
    }
}

impl Add for Pair {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row.add(rhs.row),
            col: self.col.add(rhs.col),
        }
    }
}

impl Sub for Pair {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row.sub(rhs.row),
            col: self.col.sub(rhs.col),
        }
    }
}
