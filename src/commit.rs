use object_id::ObjectId;
use object_header::ObjectHeader;
use meta::Meta;
use serialization::Serializable;
use serialization;

#[deriving(PartialEq, Show)]
pub struct Commit {
    pub meta: Meta,
    pub author_name: String,
    pub author_email: String,
    pub author_date: uint,
    pub committer_name: String,
    pub committer_email: String,
    pub commit_date: uint,
    pub message: String,
    pub tree_id: ObjectId,
    pub parent_ids: Vec<ObjectId>,
}

impl Serializable for Commit {
    fn encode(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        let mut header = self.meta.header;

        let body_buff = self.encode_body();

        // Update header length if necessary.
        if header.length == 0 {
            header = ObjectHeader { length: body_buff.len(), ..header };
        }

        let header_buff = header.encode();

        buff.push_all(header_buff.as_slice());
        buff.push_all(body_buff.as_slice());

        buff
    }

    fn encode_body(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.push_all(format!("tree {}\n", self.tree_id.hash).into_bytes().as_slice());

        for id in self.parent_ids.iter() {
            buff.push_all(format!("parent {}\n", id.hash).into_bytes().as_slice());
        }

        buff.push_all(serialization::encode_author_info(self).as_slice());
        buff.push_all(serialization::encode_commit_info(self).as_slice());

        buff.push_all(format!("\n{}", self.message).into_bytes().as_slice());

        buff
    }
}
