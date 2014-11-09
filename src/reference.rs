//! Repository references.

use object_id::ObjectId;

static LOCAL_BRANCH_PREFIX: &'static str = "/refs/heads/";
static REMOTE_TRACKING_BRANCH_PREFIX: &'static str = "/refs/remotes/";
static TAG_PREFIX: &'static str = "/refs/tags/";
static NOTE_PREFIX: &'static str = "/refs/notes/";

pub struct Reference {
    pub canonical_name: String,
    pub target_identifier: ObjectId,
    pub remote_name: Option<String>,
}