//! The maze and its generation algorithms

use core::fmt::{Debug, Display, Formatter, Result};
use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

use crate::board::{Board, CELL_STEP};
use crate::direction::Direction;
use crate::pair::{Pair, Perimeter};
use crate::stack::Stack;
use crate::tile::Tile;
use crate::visit_status::VisitStatus;
use core::convert::From;
use core::ops::{Add, Mul, Sub};

/// A maze generated by some algorithm.
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
    /// * An optional fully generated maze.
    #[inline]
    #[must_use]
    pub fn from_backtracking(height: usize, width: usize) -> Option<Self> {
        let mut board = Board::<Tile>::new(height, width);

        let mut visited = Board::<VisitStatus>::new(height, width);
        let start = Self::choose_perimeter_pair(&board)?;
        Self::add_maze_entry(start, &mut board);

        let mut visited_stack: Stack<Pair> = Stack::new();

        visited_stack.push(start.pair);
        let _: bool =
            Self::visit_and_mark_as_path(&mut board, &mut visited, start.pair)?;

        while !visited_stack.empty() {
            let popped_pair = visited_stack.top()?;

            let Some(direction) =
                Self::choose_random_unvisited_direction(popped_pair, &visited)
            else {
                visited_stack.pop();
                let _: bool = Self::visit_and_mark_as_path(
                    &mut board,
                    &mut visited,
                    popped_pair,
                )?;
                continue;
            };

            let new_pair =
                popped_pair.add(CELL_STEP.mul(Pair::from(direction)));
            visited_stack.push(new_pair);
            let _: bool = Self::visit_and_mark_as_path(
                &mut board,
                &mut visited,
                new_pair,
            )?;

            // the in-between cell should be a wall, which we can remove
            let in_between_pair = popped_pair.add(Pair::from(direction));
            let _: bool = Self::visit_and_mark_as_path(
                &mut board,
                &mut visited,
                in_between_pair,
            )?;
        }

        let end = Self::choose_perimeter_pair(&board)?;
        let _: bool =
            Self::visit_and_mark_as_path(&mut board, &mut visited, end.pair)?;
        Self::add_maze_entry(end, &mut board);

        Some(Self { board })
    }

    /// Updates the board and its visitation status against some pair.
    ///
    /// ### Parameters
    /// * `board`: The board of the maze.
    /// * `visited`: The visitation status of each tile in the maze.
    ///
    /// ### Returns
    /// * `true` if the update succeeded, otherwise `None` if there was an indexing issue.
    fn visit_and_mark_as_path(
        board: &mut Board<Tile>,
        visited: &mut Board<VisitStatus>,
        pair: Pair,
    ) -> Option<bool> {
        *board.get_mut_from_pair(pair)? = Tile::Path;
        *visited.get_mut_from_pair(pair)? = VisitStatus::Visited;

        Some(true)
    }

    /// Adds an entry point to the maze.
    ///
    /// ### Parameters
    /// * `pair`: The `Pair` adjacent to the perimeter of the board.
    /// * `board`: The board of `Tiles` to update.
    fn add_maze_entry(perimeter_tile: Perimeter, board: &mut Board<Tile>) {
        let Some(direction) = Direction::iter().find(|direction| {
            let possible_perimeter_tile = board.get_from_pair(
                perimeter_tile
                    .pair
                    .add(CELL_STEP.mul(Pair::from(*direction))),
            );
            possible_perimeter_tile.is_none()
        }) else {
            return;
        };

        let Some(cell) = board
            .get_mut_from_pair(perimeter_tile.pair.add(Pair::from(direction)))
        else {
            return;
        };

        *cell = Tile::Entry;
    }

    /// Chooses a `Pair` from the perimeter of the maze.
    ///
    /// ### Parameters
    /// * `board`: A reference to the board to get a perimeter cell from.
    ///
    /// ### Returns
    /// * An optional pair.
    fn choose_perimeter_pair(board: &Board<Tile>) -> Option<Perimeter> {
        let side = Direction::iter()
            .collect::<Vec<Direction>>()
            .choose(&mut thread_rng())
            .copied()
            .unwrap_or_default();

        let unsigned_to_signed_cell = |value: usize| {
            i32::try_from(Board::<Tile>::cell_position_to_index(value)).ok()
        };

        let pick_random_cell = |max: usize| {
            unsigned_to_signed_cell(thread_rng().gen_range(0..max))
        };

        match side {
            Direction::Down => {
                let row = unsigned_to_signed_cell(board.cell_height.sub(1))?;
                let col = pick_random_cell(board.cell_width)?;
                Some(Perimeter {
                    pair: Pair { row, col },
                })
            }
            Direction::Left => {
                let row = pick_random_cell(board.cell_height)?;
                let col = unsigned_to_signed_cell(0)?;
                Some(Perimeter {
                    pair: Pair { row, col },
                })
            }
            Direction::Up => {
                let row = unsigned_to_signed_cell(0)?;
                let col = pick_random_cell(board.cell_width)?;
                Some(Perimeter {
                    pair: Pair { row, col },
                })
            }
            Direction::Right => {
                let row = pick_random_cell(board.cell_height)?;
                let col = unsigned_to_signed_cell(board.cell_width.sub(1))?;
                Some(Perimeter {
                    pair: Pair { row, col },
                })
            }
        }
    }

    /// Choosese a random unvisited direction.
    ///
    /// ### Parameters
    /// * `pair`: A `Pair` to access a `Board` with.
    /// * `visited`: The `Board` of visitation status.
    ///
    /// ### Returns
    /// * An optional direction.
    #[inline]
    #[must_use]
    pub fn choose_random_unvisited_direction(
        pair: Pair,
        visited: &Board<VisitStatus>,
    ) -> Option<Direction> {
        let direction_choices = Self::get_unvisited_directions(pair, visited);
        direction_choices
            .into_iter()
            .collect::<Vec<Direction>>()
            .choose(&mut thread_rng())
            .copied()
    }

    /// Gets unvisited directions.
    ///
    /// ### Parameters
    /// * `pair`: A `Pair` to access a `Board` with.
    /// * `visited`: The `Board` of visitation status.
    ///
    /// ### Returns
    /// * A set of `Direction` enums.
    #[inline]
    #[must_use]
    pub fn get_unvisited_directions(
        pair: Pair,
        visited: &Board<VisitStatus>,
    ) -> HashSet<Direction> {
        Direction::iter()
            .filter(|direction| {
                let Some(visit_status_of_new_pair) = visited.get_from_pair(
                    pair.add(CELL_STEP.mul(Pair::from(*direction))),
                ) else {
                    return false;
                };
                *visit_status_of_new_pair == VisitStatus::Unvisited
            })
            .collect::<HashSet<Direction>>()
    }
}

#[cfg(test)]
mod test_maze {
    use std::collections::HashSet;

    use core::ops::Mul;
    use strum::IntoEnumIterator;

    use crate::{
        board::{Board, CELL_STEP},
        direction::Direction,
        maze::Maze,
        pair::Pair,
        visit_status::VisitStatus,
    };

    #[test]
    fn test_get_univisited_directions() {
        let pair = Pair::from_row_and_col(3, 3);
        let mut board = Board::<VisitStatus>::new(6, 6);
        let none_visited = Direction::iter().collect::<HashSet<Direction>>();
        assert_eq!(Maze::get_unvisited_directions(pair, &board), none_visited);

        if let Some(cell) = board.get_mut_from_pair(
            pair + CELL_STEP.mul(Pair::from(Direction::Left)),
        ) {
            *cell = VisitStatus::Visited;
        }
        let mut left_visited = none_visited;
        let _: bool = left_visited.remove(&Direction::Left);
        assert_eq!(Maze::get_unvisited_directions(pair, &board), left_visited);

        if let Some(cell) = board.get_mut_from_pair(
            pair + CELL_STEP.mul(Pair::from(Direction::Right)),
        ) {
            *cell = VisitStatus::Visited;
        }
        let mut left_and_right_visited = left_visited;
        let _: bool = left_and_right_visited.remove(&Direction::Right);
        assert_eq!(
            Maze::get_unvisited_directions(pair, &board),
            left_and_right_visited
        );
    }
}
