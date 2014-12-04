//! Represents Git identifiers.

use serialize::hex::ToHex;
use serialize::hex::FromHex;
use error::GitError;

pub static RAW_SIZE: uint = 20;
pub static HEX_SIZE: uint = 40;

#[deriving(PartialEq, Show, Clone)]
pub struct ObjectId {
    pub hash: String,
    bytes: Vec<u8>
}

impl ObjectId {
    pub fn from_string(hash: &str) -> Result<ObjectId, GitError> {
        let bytes = try!(FromHex::from_hex(hash));

        Ok(ObjectId {
            hash: hash.to_string(),
            bytes: bytes
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> ObjectId {
        ObjectId {
            hash: bytes.to_hex(),
            bytes: bytes.to_vec()
        }
    }
}
