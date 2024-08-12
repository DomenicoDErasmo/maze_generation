//! The maze and its generation algorithms

use core::fmt::{Debug, Display, Formatter, Result};

use crate::cell::Cell;

pub struct Maze {
    /// The grid of cells
    pub grid: Vec<Vec<Cell>>,
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

impl Maze {}
