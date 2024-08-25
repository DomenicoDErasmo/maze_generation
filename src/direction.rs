//! A module detailing directions.

/// The possible directions to move in the maze.
#[derive(
    strum_macros::EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug, Default,
)]
pub enum Direction {
    /// Going "up" (i.e. decrementing a column).
    #[default]
    Up,
    /// Going "right" (i.e. incrementing a row).
    Right,
    /// Going "down" (i.e. incrementing a column).
    Down,
    /// Going "left" (i.e. decrementing a row).
    Left,
}
