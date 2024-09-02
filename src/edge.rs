//! A module for describing connections between `Pairs`.

use crate::pair::Pair;

/// A connection between `Pairs`.
#[derive(PartialEq, Eq, Hash)]
pub struct Edge {
    /// The `Pairs` in the `Edge`.
    pub pairs: (Pair, Pair),
}
