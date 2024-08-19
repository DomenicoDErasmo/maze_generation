//! The maze and its generation algorithms

use core::fmt::{Debug, Display, Formatter, Result};
use core::ops::{Add, Mul};

use crate::tile::Tile;

pub struct Maze {
    /// The grid of cells
    pub grid: Vec<Vec<Tile>>,
}

impl Display for Maze {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut result = String::new();

        for row in &self.grid {
            for cell in row {
                result.push_str(cell.to_string().as_str());
            }
            result.push('\n');
        }

        write!(formatter, "{result}")
    }
}

impl Debug for Maze {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{self}")
    }
}

/// Denotes a pair of coordinates that are used to index a 2D-vector.
struct Pair {
    /// The row to access.
    pub i: usize,
    /// The column to access.
    pub j: usize,
}

impl Pair {
    /// Creates the `Pair`.
    #[inline]
    #[must_use]
    pub const fn from(i: usize, j: usize) -> Self {
        Self { i, j }
    }
}

impl Maze {
    /// Uses a backtracking algorithm to randomly generate a maze.
    ///
    /// ### Parmaters
    /// * `height`: The number of maze rows.
    /// * `width`: The number of maze columns.
    ///
    /// ### Returns
    /// * The fully generated maze.
    #[inline]
    #[must_use]
    pub fn from_backtracking(height: usize, width: usize) -> Self {
        let walkable_and_walls = |val: usize| val.mul(2).add(1);
        let grid = vec![
            vec![Tile::WALL; walkable_and_walls(width)];
            walkable_and_walls(height)
        ];

        Self { grid }
    }
}
