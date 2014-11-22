//! Represents Git branches.

use object_id::ObjectId;
use object_database::find_object_by_id;
use error::GitError::CorruptRepository;
use error::GitError;
use repository::Repository;
use commit::Commit;
use commit;
use eobject::EObject::ECommit;
use commit_sort_strategy::CommitSortStrategy::MostRecent;
use commit_filter::CommitFilter;

pub struct Branch {
    pub name: String,
    pub canonical_name: String,
    pub tip_id: ObjectId,
    pub tracked_branch: Option<Box<Branch>>,
    pub is_remote: bool,
    pub is_detached: bool,
}

impl Branch {
    pub fn is_tracking(&self) -> bool {
        self.tracked_branch.is_some()
    }
}

pub fn tip(repository: &Repository, branch: &Branch) -> Result<Commit, GitError> {
    match find_object_by_id(repository, &branch.tip_id) {
        Ok(box ECommit(c)) => Ok(c),
        Err(e) => Err(e),
        _ => Err(CorruptRepository("Could not find the commit the branch points to")),
    }
}

pub fn commits(repository: &Repository, branch: &Branch) -> Result<Vec<Commit>, GitError> {
    let filter = CommitFilter {
        since: Some(vec![branch.tip_id.clone()]),
        until: None,
        limit: -1,
        offset: 0,
        sort: MostRecent,
    };

    commit::find(repository, filter)
}