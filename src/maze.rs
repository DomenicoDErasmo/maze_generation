//! The maze and its generation algorithms

use core::fmt::{Debug, Display, Formatter, Result};

use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

use crate::board::Board;
use crate::direction::Direction;
use crate::pair::Pair;
use crate::stack::Stack;
use crate::tile::Tile;
use crate::visit_status::VisitStatus;
use core::convert::From;
use core::ops::Add;

pub struct Maze {
    /// The grid of cells
    pub board: Board<Tile>,
}

impl Display for Maze {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut result = String::new();

        for row in &self.board.grid {
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
        let mut grid = Board::<Tile>::new(height, width);

        for (i, row) in grid.grid.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if i % 2 == 1 && j % 2 == 1 {
                    *cell = Tile::Path;
                }
            }
        }

        let visited = Board::<VisitStatus>::new(height, width);

        let first_cell_visited = Pair::from_row_and_col(
            thread_rng()
                .gen_range(0..height)
                .try_into()
                .unwrap_or_default(),
            thread_rng()
                .gen_range(0..width)
                .try_into()
                .unwrap_or_default(),
        );

        let mut visited_stack: Stack<Pair> = Stack::from(first_cell_visited);

        while !visited_stack.empty() {
            let Some(popped_pair) = visited_stack.pop() else {
                break;
            };

            let Some(_direction) =
                choose_random_unvisited_direction(popped_pair, &visited)
            else {
                break;
            };

            // TODO: push new pair to stack and eliminate any walls between the new pair and the current pair
            // TODO: should I not pop from the stack until all adjacent cells are gone?
        }

        Self { board: grid }
    }
}

/// Choosese a random unvisited direction.
fn choose_random_unvisited_direction(
    pair: Pair,
    visited: &Board<VisitStatus>,
) -> Option<Direction> {
    let direction_choices = Direction::iter()
        .filter(|direction| {
            let Some(visit_status_of_new_pair) =
                visited.get_from_pair(pair.add(Pair::from(*direction)))
            else {
                return false;
            };
            *visit_status_of_new_pair == VisitStatus::Unvisited
        })
        .collect::<Vec<Direction>>();

    direction_choices.choose(&mut thread_rng()).copied()
}
