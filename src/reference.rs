//! Repository references.

use object_id::ObjectId;
use repository::Repository;
use reference_collection::ReferenceCollection;
use std::io::fs::File;
use std::io::fs::readdir;
use std::io::IoError;
use std::io::fs::PathExtensions;
use error::GitError::CorruptRepository;

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

    let local_references: Vec<Reference> = local_files.iter().filter(|e| e.is_file()).map(|entry| {
        Reference {
            canonical_name: entry.filename_str().unwrap().into_string(),
            target_identifier: ObjectId::from_string(File::open(entry).read_to_string().unwrap().as_slice()), // TODO: TRIM!
            remote_name: None
        }
    }).collect();

    let head_contents = try!(File::open(&repository.path.join("HEAD")).read_to_string());
    let head_name = head_contents.replace("ref: refs/heads/", "").as_slice().trim_right().into_string();
    let head = local_references.into_iter().find(|r| r.canonical_name == head_name);

    Ok(ReferenceCollection {
        head: head,
        local_references: Vec::new(),
        remote_references: Vec::new(),
    })
}