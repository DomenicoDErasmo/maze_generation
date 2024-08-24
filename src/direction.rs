//! A module detailing directions.

#[derive(
    strum_macros::EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug, Default,
)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}
