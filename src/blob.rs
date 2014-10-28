use meta::Meta;
use serialization;
use has_meta::HasMeta;
use object_header::ObjectHeader;
use error::GitError;
use object_id::ObjectId;

#[deriving(PartialEq, Show)]
pub struct Blob {
    pub meta: Meta,
    pub size: uint,
    pub contents: Vec<u8>,
}

impl HasMeta for Blob {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }
}

pub fn encode(blob: &Blob) -> Vec<u8> {
    serialization::encode(blob, blob.contents.as_slice())
}

pub fn decode(bytes: &[u8]) -> Result<Blob, GitError> {
    let (data, header) = serialization::decode(bytes);
    decode_body(data, &header)
}

fn decode_body(bytes: &[u8], header: &ObjectHeader) -> Result<Blob, GitError> {
    Ok(Blob {
        meta: Meta {
            id: ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140"),
            header: *header,
        },
        size: bytes.len(),
        contents: bytes.into_vec(),
    })
}