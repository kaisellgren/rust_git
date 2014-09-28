use object_id::ObjectId;
use pack_file::PackFile;

pub struct PackIndex {
    pub fan_out_table: Vec<uint>,
    pub object_ids: Vec<ObjectId>,
    pub offsets: Vec<uint>,
    pub length: uint,
    pub pack_file: PackFile,
}