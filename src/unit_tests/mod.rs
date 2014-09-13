extern crate flate;

use std::str;
use std::io::File;
use flate::inflate_bytes_zlib;
use commit::Commit;
use serialization::Serializable;
use object_type;
use object_header::ObjectHeader;
use object_id::ObjectId;
use meta::Meta;

#[test]
fn decode_commit() {
    // Test.
    /*let path = "C:/Projects/RustGit/.git/objects/37/eaf372e4a1a5638706b757f8a086b78a5b490b";
    let compressed_contents = File::open(&Path::new(path)).read_to_end().unwrap();
    let contents = inflate_bytes_zlib(compressed_contents.as_slice()).unwrap();
    let s = str::from_utf8(contents.as_slice()).unwrap();*/
    //let bytes = File::open(&Path::new(".gitignore")).read_to_string().unwrap();
    let c = Commit {
        meta: Meta {
            id: ObjectId::new("blabla"),
            header: ObjectHeader {
                typ: object_type::Commit,
                length: 1
            },
        },
        author_name: "Kai Sellgren",
        author_email: "kaisellgren@gmail.com",
        author_date: 0,
        committer_name: "Kai Sellgren 2",
        committer_email: "kaisellgren+2@gmail.com",
        commit_date: 2,
        message: "Hooray!",
        tree_id: ObjectId::new("test"),
        parent_ids: vec![ObjectId::new("first"), ObjectId::new("second")]
    };

    println!("{}", str::from_utf8(c.encode().as_slice()).unwrap());
    fail!("asd");
}
