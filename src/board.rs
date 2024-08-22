//! A 2D vector meant to interface cleanly with pair access.

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
    #[inline]
    #[must_use]
    pub fn new(height: usize, width: usize) -> Self {
        let walkable_and_walls = |val: usize| val.mul(2).add(1);
        Self {
            grid: vec![
                vec![T::default(); walkable_and_walls(width)];
                walkable_and_walls(height)
            ],
        }
    }
}

impl<T> Board<T>
where
    T: Sized,
{
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
