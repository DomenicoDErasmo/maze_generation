//! Marks whether the tile was visited or not.

/// Denotes whether the cell has been visited or not.
#[derive(Clone, PartialEq, Eq, Default)]
pub enum VisitStatus {
    /// The cell has not been visited yet.
    #[default]
    Unvisited,
    /// The cell was previously visited.
    Visited,
}
