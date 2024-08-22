//! A module detailing directions.

#[derive(strum_macros::EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
