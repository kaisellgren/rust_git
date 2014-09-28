use object_id::ObjectId;
use tag::Tag;
use branch::Branch;
use reference::Reference;
use pack_index::PackIndex;

pub struct Repository {
    pub path: String,
    pub wc_path: String,
    pub tags: Vec<Tag>,
    pub branches: Vec<Branch>,
    pub references: Vec<Reference>,
    pub pack_indexes: Vec<PackIndex>,
}