//! Utilities for working with the repository.

use std::io::File;
use std::io::fs::PathExtensions;
use commit;
use git_object::GitObject;
use git_object::GitObject::GitCommit;
use object_type::ObjectType;
use object_header;
use object_id::ObjectId;
use error::GitError;
use error::GitError::NotFound;
use error::GitError::CorruptObject;
use flate::inflate_bytes_zlib;
use repository::Repository;

pub fn find_object_by_id(repository: &Repository, id: &ObjectId) -> Result<Box<GitObject>, GitError> {
    let part1 = id.hash.as_slice().slice_to(2);
    let part2 = id.hash.as_slice().slice_from(2);
    let path = Path::new(format!("{}objects/{}/{}", repository.path.dirname(), part1, part2));

    if path.exists() {
        let mut file = File::open(&path);
        let bytes = try!(file.read_to_end());
        let data = match inflate_bytes_zlib(bytes.as_slice()) {
            Some(d) => d,
            None => return Err(CorruptObject(
                format!("Could not inflate data at {}", path.display()).into_cow()
            ))
        };

        let header = try!(object_header::decode(data.as_slice()));
        let object_data = data.as_slice().slice_from(data.len() - header.length);

        match header.typ {
            ObjectType::Commit => commit::decode_body(object_data, &header).map(|c| box GitCommit(c)),
            _ => Err(NotFound),
        }
    } else {
        Err(NotFound)
    }
}