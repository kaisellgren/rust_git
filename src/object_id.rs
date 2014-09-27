#[deriving(PartialEq, Show)]
pub struct ObjectId {
    pub hash: &'static str,
    bytes: Vec<u8>
}

impl ObjectId {
    pub fn new(hash: &'static str) -> ObjectId {
        ObjectId {
            hash: hash,
            bytes: Vec::new()
        }
    }
}
