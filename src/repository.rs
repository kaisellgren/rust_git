//! Represents the actual Git repository.

use tag::Tag;
use branch::Branch;
use reference::Reference;
use pack_index::PackIndex;
use std::io::fs::mkdir_recursive;
use std::io::FilePermission;
use std::io::fs::PathExtensions;
use file_util::create_file_with_contents;
use error::GitError;
use result::EmptyResult;

pub struct Repository {
    pub path: Path,
    pub wc_path: Path,
    pub tags: Vec<Tag>,
    pub branches: Vec<Branch>,
    pub references: Vec<Reference>,
    pub pack_indexes: Vec<PackIndex>,
    pub bare: bool,
}

impl Repository {
    pub fn open(path: &Path) -> Result<Repository, GitError> {
        let mut repository_path = path.clone();
        repository_path.push(".git/");

        try!(init(&repository_path));

        Ok(Repository {
            path: repository_path,
            wc_path: path.clone(),
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
            bare: false,
        })
    }

    pub fn open_bare(path: &Path) -> Result<Repository, GitError> {
        let repository_path = path.clone();

        try!(init(&repository_path));

        Ok(Repository {
            path: repository_path,
            wc_path: path.clone(),
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
            bare: true,
        })
    }
}

fn init(repository_path: &Path) -> EmptyResult {
    let ensure_folders_exist = || -> EmptyResult {
        let paths = vec!["objects/pack", "objects/info", "refs/heads", "refs/remotes", "refs/tags",
            "refs/tags"];

        for p in paths.iter() {
            try!(mkdir_recursive(&repository_path.join(*p), FilePermission::all()));
        }

        Ok(())
    };

    let create_default_files = || -> EmptyResult {
        try!(create_file_with_contents(&repository_path.join("HEAD"), b"ref: refs/heads/master\n"));
        try!(create_file_with_contents(&repository_path.join("description"), b"Unnamed repository; edit\n
this file 'description' to name the repository.\n"));

        // TODO: Let's implement a Config struct or something.
        Ok(try!(create_file_with_contents(&repository_path.join("config"), b"[core]\nrepositoryformatversion\n
= 0\nfilemode = false\nbare = false\nlogallrefupdates = true\nsymlinks = false\nignorecase = true\nhideDotFiles = dotGitOnly")))
    };

    try!(ensure_folders_exist());

    if !is_initialized(repository_path) {
        try!(create_default_files());
    }

    Ok(())
}

fn is_initialized(repository_path: &Path) -> bool {
    repository_path.join("HEAD").exists()
}