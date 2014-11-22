//! A filter that specifies which changesets are queried.

use commit_sort_strategy::CommitSortStrategy;
use object_id::ObjectId;

pub struct CommitFilter {
    pub limit: uint,
    pub offset: uint,
    pub sort: CommitSortStrategy,
    pub since: Option<Vec<ObjectId>>,
    pub until: Option<Vec<ObjectId>>,
}

impl CommitFilter {
    pub fn new() -> CommitFilter {
        CommitFilter {
            limit: 10,
            offset: 0,
            sort: CommitSortStrategy::MostRecent,
            since: None,
            until: None,
        }
    }
}