use repository::Repository;
use std::io::TempDir;
use std::io::fs::rmdir_recursive;

#[test]
fn can_create_new() {
    let path = Path::new("resources/repositories/non-existant/.");
    let repo = Repository::open_bare(path.as_str().unwrap());
    rmdir_recursive(&path);
}