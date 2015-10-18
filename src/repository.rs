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
use config::config::Config;

pub struct Repository {
    pub path: Path,
    pub wc_path: Path,
    pub tags: Vec<Tag>,
    pub branches: Vec<Branch>,
    pub references: Vec<Reference>,
    pub pack_indexes: Vec<PackIndex>,
    pub bare: bool,
    pub config: Config,
}

impl Repository {
    pub fn open(path: &Path) -> Result<Repository, GitError> {
        let mut repository_path = path.clone();
        repository_path.push(".git/");

        let repo = Repository {
            config: Config::from_path(&repository_path),
            path: repository_path,
            wc_path: path.clone(),
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
            bare: false,
        };

        try!(init(&repo));

        Ok(repo)
    }

    pub fn open_bare(path: &Path) -> Result<Repository, GitError> {
        let repository_path = path.clone();

        let repo = Repository {
            config: Config::from_path(&repository_path),
            path: repository_path,
            wc_path: path.clone(),
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
            bare: true,
        };

        try!(init(&repo));

        Ok(repo)
    }
}

fn init(repo: &Repository) -> EmptyResult {
    let ensure_folders_exist = || -> EmptyResult {
        let paths = vec!["objects/pack", "objects/info", "refs/heads", "refs/remotes", "refs/tags",
            "refs/tags"];

        for p in paths.iter() {
            try!(mkdir_recursive(&repo.path.join(*p), FilePermission::all()));
        }

        Ok(())
    };

    let create_default_files = || -> EmptyResult {
        try!(create_file_with_contents(&repo.path.join("HEAD"), b"ref: refs/heads/master\n"));
        try!(create_file_with_contents(&repo.path.join("description"), b"Unnamed repository; edit\n
this file 'description' to name the repository.\n"));

        repo.config.set_local("core", "repositoryformatversion", "0");
        Ok(try!(create_file_with_contents(&repo.path.join("config"), b"[core]\nrepositoryformatversion\n
= 0\nfilemode = false\nbare = false\nlogallrefupdates = true\nsymlinks = false\nignorecase = true\nhideDotFiles = dotGitOnly")))
    };

    try!(ensure_folders_exist());

    if !is_initialized(&repo.path) {
        try!(create_default_files());
    }

    Ok(())
}

fn is_initialized(repository_path: &Path) -> bool {
    repository_path.join("HEAD").exists()
}