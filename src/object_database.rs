use std::io::File;
use std::str;
use commit;
use commit::Commit;
use object_type;
use eobject;
use eobject::EObject;
use eobject::ECommit;
use object_type::ObjectType;
use object_header;
use object_id::ObjectId;
use error::GitError;
use error::NotFound;
use flate::inflate_bytes_zlib;
use object_header::ObjectHeader;
use repository::Repository;

pub fn find_object_by_id(repository: &Repository, id: &ObjectId) -> Result<Box<EObject>, GitError> {
    let part1 = id.hash.as_slice().slice_to(2);
    let part2 = id.hash.as_slice().slice_from(2);
    let path = Path::new(format!("{}objects/{}/{}", repository.path.dirname(), part1, part2));

    if path.exists() {
        let mut file = File::open(&path);
        let bytes = file.read_to_end().unwrap();
        let data = inflate_bytes_zlib(bytes.as_slice()).unwrap();

        let header = object_header::decode(data.as_slice());
        let object_data = data.as_slice().slice_from(data.len() - header.length);

        match header.typ {
            object_type::Commit => commit::decode_body(object_data, &header).map(|c| box ECommit(c)),
            object_type::Blob => Err(NotFound),
            object_type::Tag => Err(NotFound),
            object_type::Note => Err(NotFound),
            object_type::Tree => Err(NotFound),
        }
    } else {
        Err(NotFound)
    }
}