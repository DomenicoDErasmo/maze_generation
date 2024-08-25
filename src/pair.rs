//! A pair of co-ordinates to enable easy vector access.

use core::ops::{Add, Mul, Sub};

use crate::direction::Direction;

/// Denotes a row-and-column pair to access a 2-D vector.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    /// Converts a direction into a `Pair`.
    ///
    /// ### Parameters
    /// * `direction`: The direction to convert.
    ///
    /// ### Returns
    /// * The `Pair` relating to the direction.
    #[inline]
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Down => Self::from_row_and_col(1, 0),
            Direction::Left => Self::from_row_and_col(0, -1),
            Direction::Up => Self::from_row_and_col(-1, 0),
            Direction::Right => Self::from_row_and_col(0, 1),
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

impl Mul<i32> for Pair {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            row: self.row.mul(rhs),
            col: self.col.mul(rhs),
        }
    }
}

impl Mul<Pair> for i32 {
    type Output = Pair;
    #[inline]
    fn mul(self, rhs: Pair) -> Self::Output {
        Self::Output {
            row: rhs.row.mul(self),
            col: rhs.col.mul(self),
        }
    }
}

#[cfg(test)]
mod pair_tets {
    use crate::{direction::Direction, pair::Pair};

    use core::ops::{Add, Mul, Sub};

    #[test]
    fn test_add() {
        let lhs_pair = Pair::from_row_and_col(1, -1);
        let rhs_pair = Pair::from_row_and_col(4, -3);
        assert_eq!(lhs_pair.add(rhs_pair), Pair::from_row_and_col(5, -4));
    }

    #[test]
    fn test_sub() {
        let lhs_pair = Pair::from_row_and_col(1, -1);
        let rhs_pair = Pair::from_row_and_col(4, -3);
        assert_eq!(lhs_pair.sub(rhs_pair), Pair::from_row_and_col(-3, 2));
    }

    #[test]
    fn test_mul() {
        let pair_mul_from_rhs = Pair::from_row_and_col(4, -4);
        assert_eq!(2_i32.mul(pair_mul_from_rhs), Pair::from_row_and_col(8, -8));

        let pair_mul_from_lhs = Pair::from_row_and_col(-1, 1);
        assert_eq!(pair_mul_from_lhs.mul(3_i32), Pair::from_row_and_col(-3, 3));
    }

    #[test]
    fn test_direction() {
        let left = Pair::from(Direction::Left);
        assert_eq!(left, Pair::from_row_and_col(0, -1));

        let up = Pair::from(Direction::Up);
        assert_eq!(up, Pair::from_row_and_col(1, 0));

        let right = Pair::from(Direction::Right);
        assert_eq!(right, Pair::from_row_and_col(0, 1));

        let down = Pair::from(Direction::Down);
        assert_eq!(down, Pair::from_row_and_col(-1, 0));
    }
}
