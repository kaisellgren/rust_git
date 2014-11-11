//! Repository references.

use object_id::ObjectId;
use repository::Repository;
use reference_collection::ReferenceCollection;
use std::io::fs::File;
use std::io::fs::readdir;
use std::io::IoError;

static LOCAL_BRANCH_PREFIX:           &'static str = "refs/heads/";
static REMOTE_TRACKING_BRANCH_PREFIX: &'static str = "refs/remotes/";
static TAG_PREFIX:                    &'static str = "refs/tags/";
static NOTE_PREFIX:                   &'static str = "refs/notes/";

#[deriving(PartialEq, Show, Clone)]
pub struct Reference {
    pub canonical_name: String,
    pub target_identifier: ObjectId,
    pub remote_name: Option<String>,
}

pub fn find(repository: &Repository) -> Result<ReferenceCollection, IoError> {
    let local_files = try!(readdir(&repository.path.join(LOCAL_BRANCH_PREFIX)));

    let local_references: Vec<Option<Reference>> = local_files.iter().map(|entry| {
        if entry.is_file() {
            Some(Reference {
                canonical_name: entry.filename_str().unwrap().into_string(),
                target_identifier: ObjectId::from_string(File::open(entry).read_to_string().unwrap().as_slice()), // TODO: TRIM!
                remote_name: None
            })
        } else {
            None
        }
    }).collect();

    Ok(ReferenceCollection {
        head: None,
        local_references: Vec::new(),
        remote_references: Vec::new(),
    })
}