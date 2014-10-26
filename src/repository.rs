use object_id::ObjectId;
use tag::Tag;
use branch::Branch;
use reference::Reference;
use pack_index::PackIndex;

pub struct Repository {
    pub path: Path,
    pub wc_path: Path,
    pub tags: Vec<Tag>,
    pub branches: Vec<Branch>,
    pub references: Vec<Reference>,
    pub pack_indexes: Vec<PackIndex>,
}

impl Repository {
    pub fn open(path: &str) -> Repository {
        let wc_path = Path::new(path);

        let mut repository_path = Path::new(path);
        repository_path.push(".git/");

        Repository {
            path: repository_path,
            wc_path: wc_path,
            tags: Vec::new(),
            branches: Vec::new(),
            references: Vec::new(),
            pack_indexes: Vec::new(),
        }
    }
}