//! A filter that specifies which changesets are queried.

use commit_sort_strategy;
use object_id::ObjectId;

pub struct CommitFilter {
    pub limit: uint,
    pub offset: uint,
    pub sort: commit_sort_strategy::CommitSortStrategy,
    pub since: Option<Vec<ObjectId>>,
    pub until: Option<Vec<ObjectId>>,
}

impl CommitFilter {
    pub fn new() -> CommitFilter {
        CommitFilter {
            limit: 10,
            offset: 0,
            sort: commit_sort_strategy::MostRecent,
            since: None,
            until: None,
        }
    }
}