//! A module detailing directions.

#[derive(strum_macros::EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
