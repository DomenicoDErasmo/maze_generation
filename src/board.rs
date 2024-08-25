//! A 2D vector meant to interface cleanly with pair access.

use core::fmt::{Debug, Display, Formatter, Result};
use core::ops::{Add, Mul};

use crate::pair::Pair;

pub struct Board<T>
where
    T: Sized,
{
    pub grid: Vec<Vec<T>>,
}

impl<T> Board<T>
where
    T: Clone + Default + Sized,
{
    /// Creates a `Board` from some numbers of horizontal and vertical "cells".
    ///
    /// ### Parameters
    /// * `rows`: The number of row "cells" to instantiate board.
    /// * `cols`: The number of column "cells" to instantiate the board.
    ///
    /// ### Returns
    /// A fully empty `Board`.  
    #[inline]
    #[must_use]
    pub fn new(rows: usize, width: usize) -> Self {
        let walkable_and_walls = |val: usize| val.mul(2).add(1);
        Self {
            grid: vec![
                vec![T::default(); walkable_and_walls(width)];
                walkable_and_walls(rows)
            ],
        }
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
        let Ok(row_index) = usize::try_from(pair.row) else {
            return None;
        };
        let row = self.grid.get(row_index)?;

        let Ok(col_index) = usize::try_from(pair.col) else {
            return None;
        };

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
        let Ok(row_index) = usize::try_from(pair.row) else {
            return None;
        };
        let row = self.grid.get_mut(row_index)?;

        let Ok(col_index) = usize::try_from(pair.col) else {
            return None;
        };

        row.get_mut(col_index)
    }
}
