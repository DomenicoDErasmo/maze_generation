//! The maze and its generation algorithms

use core::fmt::{Debug, Display, Formatter, Result};
use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

use crate::board::Board;
use crate::direction::Direction;
use crate::pair::Pair;
use crate::stack::Stack;
use crate::tile::Tile;
use crate::visit_status::VisitStatus;
use core::convert::From;
use core::ops::{Add, Div, Mul, Sub};

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
    pub fn from_backtracking(height: usize, width: usize) -> Option<Self> {
        let mut grid = Board::<Tile>::new(height, width);

        for (i, row) in grid.grid.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if i % 2 == 1 && j % 2 == 1 {
                    *cell = Tile::Path;
                }
            }
        }

        let mut visited = Board::<VisitStatus>::new(height, width);

        let first_cell_visited = choose_perimeter_pair(&grid)?;
        println!("{first_cell_visited:#?}");

        let mut visited_stack: Stack<Pair> = Stack::from(first_cell_visited);
        *visited.get_mut_from_pair(first_cell_visited)? = VisitStatus::Visited;

        while !visited_stack.empty() {
            let Some(popped_pair) = visited_stack.top() else {
                break;
            };

            let Some(direction) =
                choose_random_unvisited_direction(popped_pair, &visited)
            else {
                visited_stack.pop();
                continue;
            };

            let new_pair = popped_pair.add(2_i32.mul(Pair::from(direction)));
            visited_stack.push(new_pair);

            if let Some(cell) = visited.get_mut_from_pair(new_pair) {
                *cell = VisitStatus::Visited;
            }

            // the in-between cell should be a wall, which we can remove
            let in_between_pair = popped_pair.add(Pair::from(direction));
            if let Some(cell) = grid.get_mut_from_pair(in_between_pair) {
                *cell = Tile::Path;
            }
            if let Some(cell) = visited.get_mut_from_pair(in_between_pair) {
                *cell = VisitStatus::Visited;
            }
        }

        Some(Self { board: grid })
    }
}

/// Chooses a `Pair` from the perimeter of the maze.
fn choose_perimeter_pair(board: &Board<Tile>) -> Option<Pair> {
    let side = Direction::iter()
        .collect::<Vec<Direction>>()
        .choose(&mut thread_rng())
        .copied()
        .unwrap_or_default();

    match side {
        Direction::Down => {
            let last_row = board.grid.last()?;
            let Ok(row) = i32::try_from(board.grid.len().sub(2)) else {
                return None;
            };
            // row of length 0 to 41 should use cells 1-39
            // we want odds, so we generate with thread_rng().gen_range(0..19) * 2 + 1
            let Ok(col) = i32::try_from(
                thread_rng()
                    .gen_range(0..last_row.len().sub(2).div(2))
                    .mul(2)
                    .add(1),
            ) else {
                return None;
            };
            Some(Pair { row, col })
        }
        Direction::Left => {
            let first_col = board.grid.first()?;
            let Ok(row) = i32::try_from(
                thread_rng()
                    .gen_range(0..first_col.len().sub(2).div(2))
                    .mul(2)
                    .add(1),
            ) else {
                return None;
            };
            let col = 1;
            Some(Pair { row, col })
        }
        Direction::Up => {
            let first_row = board.grid.first()?;
            let row = 1;
            let Ok(col) = i32::try_from(
                thread_rng()
                    .gen_range(0..first_row.len().sub(2).div(2))
                    .mul(2)
                    .add(1),
            ) else {
                return None;
            };
            Some(Pair { row, col })
        }
        Direction::Right => {
            let Ok(row) = i32::try_from(
                thread_rng()
                    .gen_range(0..board.grid.len().sub(2).div(2))
                    .mul(2)
                    .add(1),
            ) else {
                return None;
            };
            let Ok(col) = i32::try_from(board.grid.len().sub(2)) else {
                return None;
            };
            Some(Pair { row, col })
        }
    }
}

/// Choosese a random unvisited direction.
#[inline]
#[must_use]
pub fn choose_random_unvisited_direction(
    pair: Pair,
    visited: &Board<VisitStatus>,
) -> Option<Direction> {
    let direction_choices = get_unvisited_directions(pair, visited);
    direction_choices
        .into_iter()
        .collect::<Vec<Direction>>()
        .choose(&mut thread_rng())
        .copied()
}

#[inline]
#[must_use]
pub fn get_unvisited_directions(
    pair: Pair,
    visited: &Board<VisitStatus>,
) -> HashSet<Direction> {
    Direction::iter()
        .filter(|direction| {
            let Some(visit_status_of_new_pair) = visited
                .get_from_pair(pair.add(2_i32.mul(Pair::from(*direction))))
            else {
                return false;
            };
            *visit_status_of_new_pair == VisitStatus::Unvisited
        })
        .collect::<HashSet<Direction>>()
}

#[cfg(test)]
mod test_maze {
    use std::collections::HashSet;

    use core::ops::Mul;
    use strum::IntoEnumIterator;

    use crate::{
        board::Board, direction::Direction, pair::Pair,
        visit_status::VisitStatus,
    };

    use super::get_unvisited_directions;

    #[test]
    fn test_get_univisited_directions() {
        let pair = Pair::from_row_and_col(3, 3);
        let mut board = Board::<VisitStatus>::new(6, 6);
        let none_visited = Direction::iter().collect::<HashSet<Direction>>();
        assert_eq!(get_unvisited_directions(pair, &board), none_visited);

        println!("{:#?}", pair + 2_i32.mul(Pair::from(Direction::Left)));
        if let Some(cell) = board
            .get_mut_from_pair(pair + 2_i32.mul(Pair::from(Direction::Left)))
        {
            *cell = VisitStatus::Visited;
        }
        let mut left_visited = none_visited;
        let _: bool = left_visited.remove(&Direction::Left);
        assert_eq!(get_unvisited_directions(pair, &board), left_visited);

        if let Some(cell) = board
            .get_mut_from_pair(pair + 2_i32.mul(Pair::from(Direction::Right)))
        {
            *cell = VisitStatus::Visited;
        }
        let mut left_and_right_visited = left_visited;
        let _: bool = left_and_right_visited.remove(&Direction::Right);
        assert_eq!(
            get_unvisited_directions(pair, &board),
            left_and_right_visited
        );
    }
}
