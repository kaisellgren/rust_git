//! Repository references.

use object_id::ObjectId;
use repository::Repository;
use reference_collection::ReferenceCollection;
use std::io::fs::File;
use std::io::fs::readdir;
use std::io::fs::PathExtensions;
use error::GitError;
use error::GitError::NotFound;

static LOCAL_BRANCH_PREFIX:           &'static str = "refs/heads/";
static REMOTE_TRACKING_BRANCH_PREFIX: &'static str = "refs/remotes/";
static TAG_PREFIX:                    &'static str = "refs/tags/";
static NOTE_PREFIX:                   &'static str = "refs/notes/";

#[deriving(PartialEq, Show, Clone)]
pub struct Reference {
    pub target_identifier: ObjectId,
    pub canonical_name:    String,
    pub remote_name:       Option<String>,
}

pub fn find(repository: &Repository) -> Result<ReferenceCollection, GitError> {
    let local_files = try!(readdir(&repository.path.join(LOCAL_BRANCH_PREFIX)));

    let local_references: Vec<Reference> = try!(local_files.iter()
        .filter(|e| e.is_file())
        .map(path_to_reference)
        .collect()
    );

    let head_contents = try!(File::open(&repository.path.join("HEAD")).read_to_string());
    let head_name = head_contents.replace("ref: refs/heads/", "").as_slice().trim_right().into_string();
    let head = local_references.into_iter().find(|r| r.canonical_name == head_name);

    Ok(ReferenceCollection {
        head: head,
        local_references: Vec::new(),
        remote_references: Vec::new(),
    })
}

fn path_to_reference(path: &Path) -> Result<Reference, GitError> {
    let target_identifier = try!(File::open(path).read_to_string());
    Ok(Reference {
        canonical_name: try!(path.filename_str().ok_or(NotFound)).into_string(),
        target_identifier: try!(ObjectId::from_string(target_identifier.as_slice().trim())),
        remote_name: None
    })
}