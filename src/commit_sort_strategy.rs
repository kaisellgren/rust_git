//! Specifies in which order changesets are queried.

pub enum CommitSortStrategy {
    TopoLogical,
    MostRecent,
    TopologicalTime,
    Reverse,
}