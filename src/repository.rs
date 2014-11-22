//! Represents the actual Git repository.

use tag::Tag;
use branch::Branch;
use reference::Reference;
use pack_index::PackIndex;
use std::io::fs::mkdir_recursive;
use std::io::FilePermission;
use std::io::fs::PathExtensions;
use file_util::create_file_with_contents;

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
    pub fn open(path: &Path) -> Repository {
        let mut repository_path = path.clone();
        repository_path.push(".git/");

        init(&repository_path);

        Repository {
            path: repository_path,
            wc_path: path.clone(),
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
            bare: false,
        }
    }

    pub fn open_bare(path: &Path) -> Repository {
        let repository_path = path.clone();

        init(&repository_path);

        Repository {
            path: repository_path,
            wc_path: path.clone(),
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
            bare: true,
        }
    }
}

fn init(repository_path: &Path) {
    let ensure_folders_exist = || {
        let paths = vec!["objects/pack", "objects/info", "refs/heads", "refs/remotes", "refs/tags",
            "refs/tags"];

        for p in paths.iter() {
            mkdir_recursive(&repository_path.join(*p), FilePermission::all());
        }
    };

    let create_default_files = || {
        create_file_with_contents(&repository_path.join("HEAD"), b"ref: refs/heads/master\n");
        create_file_with_contents(&repository_path.join("description"), b"Unnamed repository; edit\n
this file 'description' to name the repository.\n");

        // TODO: Let's implement a Config struct or something.
        create_file_with_contents(&repository_path.join("config"), b"[core]\nrepositoryformatversion\n
= 0\nfilemode = false\nbare = false\nlogallrefupdates = true\nsymlinks = false\nignorecase = true\nhideDotFiles = dotGitOnly");
    };

    ensure_folders_exist();

    if !is_initialized(repository_path) {
        create_default_files();
    }
}

fn is_initialized(repository_path: &Path) -> bool {
    repository_path.join("HEAD").exists()
}