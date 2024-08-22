//! Marks whether the tile was visited or not.

/// Denotes whether the cell has been visited or not.
#[derive(Clone, PartialEq, Eq)]
pub enum VisitStatus {
    /// The cell has not been visited yet.
    Unvisited,
    /// The cell was previously visited.
    Visited,
}

impl Default for VisitStatus {
    #[inline]
    fn default() -> Self {
        Self::Unvisited
    }
}
