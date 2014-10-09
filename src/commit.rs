use object_id::ObjectId;
use object_header::ObjectHeader;
use object_header;
use object_type;
use meta::Meta;
use serialization::Serializable;
use serialization;
use reader::Reader;
use error::GitError;
use error::CorruptCommit;

#[deriving(PartialEq, Show, Clone)]
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

pub fn decode(bytes: &[u8]) -> Result<Commit, GitError> {
    let header = object_header::decode(bytes);

    decode_body(bytes.slice_from(bytes.len() - header.length), &header)
}

pub fn decode_body(bytes: &[u8], header: &ObjectHeader) -> Result<Commit, GitError> {
    let mut reader = Reader::from_data(bytes);

    if reader.take_string(5) != "tree " {
        return Err(CorruptCommit("Expected 'tree '"))
    }

    let tree_id = reader.take_string_based_object_id();

    reader.skip(1);

    let mut parent_ids = Vec::new();

    while reader.take_string_while(|&c| c != 32) == "parent" {
        reader.skip(1);

        parent_ids.push(reader.take_string_based_object_id());

        reader.skip(1);
    }

    reader.back(6);

    if reader.take_string(7) != "author " {
        return Err(CorruptCommit("Expected 'author '"))
    }

    let (author_name, author_email, author_date) = serialization::decode_user_info(&mut reader);

    if reader.take_string(10) != "committer " {
        return Err(CorruptCommit("Expected 'committer '"))
    }

    let (committer_name, committer_email, commit_date) = serialization::decode_user_info(&mut reader);

    reader.skip(1);

    let message = reader.get_rest_as_string();

    Ok(Commit {
        meta: Meta {
            id: ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140"),
            header: *header,
        },
        author_name: author_name,
        author_email: author_email,
        author_date: author_date,
        committer_name: committer_name,
        committer_email: committer_email,
        commit_date: commit_date,
        message: message.into_string(),
        tree_id: tree_id,
        parent_ids: parent_ids
    })
}