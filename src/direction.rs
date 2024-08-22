//! A module detailing directions.

#[derive(strum_macros::EnumIter, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
