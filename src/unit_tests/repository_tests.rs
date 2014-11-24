use repository::Repository;
use std::io::fs::rmdir_recursive;
use reference;
use error::GitError;
use result::EmptyResult;

fn open() -> Result<Repository, GitError> {
    Repository::open_bare(&Path::new("resources/repositories/one"))
}

#[test]
fn can_create_new() {
    let path = Path::new("resources/repositories/non-existant/.");
    Repository::open_bare(&path);
    rmdir_recursive(&path);
}

#[test]
fn can_find_references() {
    let repo = open().unwrap();
    reference::find(&repo).unwrap();
}