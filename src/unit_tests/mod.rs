extern crate flate;

use std::str;
use std::io::File;
use flate::inflate_bytes_zlib;
use commit::Commit;
use commit;
use serialization::Serializable;
use object_type;
use object_header::ObjectHeader;
use object_id::ObjectId;
use meta::Meta;
use std::io::fs;
use std::num::FromStrRadix;
use conversion;

#[test]
fn decode_commit() {
    // Test.
    /*let path = "C:/Projects/RustGit/.git/objects/37/eaf372e4a1a5638706b757f8a086b78a5b490b";
    let compressed_contents = File::open(&Path::new(path)).read_to_end().unwrap();
    let contents = inflate_bytes_zlib(compressed_contents.as_slice()).unwrap();
    let s = str::from_utf8(contents.as_slice()).unwrap();*/
    //let bytes = File::open(&Path::new(".gitignore")).read_to_string().unwrap();
    let c = Commit {
fn create_test_commit() -> Commit {
    Commit {
        meta: Meta {
            id: ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140"),
            header: ObjectHeader {
                typ: object_type::Commit,
                length: 0
            },
        },
        author_name: "Kai".into_string(),
        author_email: "kaisellgren@gmail.com".into_string(),
        author_date: 1388624645,
        committer_name: "Kai 2".into_string(),
        committer_email: "kaisellgren+foo@gmail.com".into_string(),
        commit_date: 1388624646,
        message: "foo bar baz qux".into_string(),
        tree_id: ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140"),
        parent_ids: vec![ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741141"), ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741142")]
    }
}

#[test]
fn encode_commit() {
    /*let path = "C:/Projects/RustGit/.git/objects/37/eaf372e4a1a5638706b757f8a086b78a5b490b";
    let compressed_contents = File::open(&Path::new(path)).read_to_end().unwrap();
    let contents = inflate_bytes_zlib(compressed_contents.as_slice()).unwrap();
    let s = str::from_utf8(contents.as_slice()).unwrap();
    println!("{}", s);*/

    //let bytes = File::open(&Path::new(".gitignore")).read_to_string().unwrap();

    let expected = File::open(&Path::new("resources/tests/commit")).read_to_end().unwrap();
    let encoded = create_test_commit().encode();

    assert_eq!(expected, encoded);
}

#[test]
fn decode_commit() {
    let commit = create_test_commit();

    let contents = File::open(&Path::new("resources/tests/commit")).read_to_end().unwrap();
    let decoded = commit::decode(contents.as_slice()).unwrap();

    assert_eq!(commit, decoded);

    fail!("STOP");
}