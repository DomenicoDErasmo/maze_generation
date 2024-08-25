//! A 2D vector meant to interface cleanly with pair access.

use core::fmt::{Debug, Display, Formatter, Result};
use core::ops::{Add, Mul};

use crate::pair::Pair;

/// The number of tiles to jump to get to the next cell.
pub const CELL_STEP: i32 = 2_i32;

pub struct Board<T>
where
    T: Sized,
{
    /// The grid of values on the board.
    pub grid: Vec<Vec<T>>,
    /// The number of "cells" (odd-indexed values) per row.
    pub cell_width: usize,
    /// The number of "cells" (odd-indexed values) per column.
    pub cell_height: usize,
}

impl<T> Board<T>
where
    T: Clone + Default + Sized,
{
    /// Creates a `Board` from some numbers of horizontal and vertical "cells".
    ///
    /// ### Parameters
    /// * `height`: The number of row "cells" to instantiate board.
    /// * `width`: The number of column "cells" to instantiate the board.
    ///
    /// ### Returns
    /// A fully empty `Board`.  
    #[inline]
    #[must_use]
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            grid: vec![
                vec![T::default(); Self::cell_position_to_index(height)];
                Self::cell_position_to_index(width)
            ],
            cell_width: width,
            cell_height: height,
        }
    }

    /// Converts the cell position to an index.
    ///
    /// ### Parameters
    /// * `position`: The value to convert.
    ///
    /// ### Returns
    /// * The index relating to the cell position.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::board::Board;
    ///
    /// assert_eq!(Board::<i32>::cell_position_to_index(4), 9);
    /// ```
    #[inline]
    #[must_use]
    pub fn cell_position_to_index(position: usize) -> usize {
        position.mul(CELL_STEP as usize).add(1)
    }
}

impl<T> Display for Board<T>
where
    T: Display,
{
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

impl<T> Debug for Board<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{self}")
    }
}

impl<T> Board<T>
where
    T: Sized,
{
    /// Gets an immutable reference to a board based on some pair.
    ///
    /// ### Parameters
    /// * `pair`: The `Pair` object used to access the board.
    ///
    /// ### Returns
    /// * An optional immutable reference to a cell in the board.
    #[inline]
    #[must_use]
    pub fn get_from_pair(&self, pair: Pair) -> Option<&T> {
        let row_index = usize::try_from(pair.row).ok()?;
        let row = self.grid.get(row_index)?;

        let col_index = usize::try_from(pair.col).ok()?;
        row.get(col_index)
    }

    /// Gets a mutable reference to a board based on some pair.
    ///
    /// ### Parameters
    /// * `pair`: The `Pair` object used to access the board.
    ///
    /// ### Returns
    /// * An optional mutable reference to a cell in the board.
    #[inline]
    #[must_use]
    pub fn get_mut_from_pair(&mut self, pair: Pair) -> Option<&mut T> {
        let row_index = usize::try_from(pair.row).ok()?;
        let row = self.grid.get_mut(row_index)?;

        let col_index = usize::try_from(pair.col).ok()?;
        row.get_mut(col_index)
    }
}
