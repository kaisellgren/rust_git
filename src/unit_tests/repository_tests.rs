use repository::Repository;
use std::io::TempDir;
use std::io::fs::rmdir_recursive;
use reference;

fn open() -> Repository {
    Repository::open_bare(&Path::new("resources/repositories/one"))
}

#[test]
fn can_create_new() {
    let path = Path::new("resources/repositories/non-existant/.");
    let repo = Repository::open_bare(&path);
    rmdir_recursive(&path);
}

#[test]
fn can_find_references() {
    let path = Path::new(".");
    let repo = open();
    reference::find(&repo).unwrap();
}