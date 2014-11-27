#![allow(unused_imports)]

use std::io::File;
use commit::Commit;
use commit;
use object_type::ObjectType;
use object_header::ObjectHeader;
use object_id::ObjectId;
use meta::Meta;

fn create_test_commit() -> Commit {
    Commit {
        meta: Meta {
            id: ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140"),
            header: ObjectHeader {
                typ: ObjectType::Commit,
                length: 271
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
        parent_ids: vec![
            ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741141"),
            ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741142")
        ]
    }
}

#[test]
fn encode_commit() {
    let expected = File::open(&Path::new("resources/tests/commit")).read_to_end().unwrap();
    let encoded = commit::encode(&create_test_commit());

    assert_eq!(expected, encoded);
}

#[test]
fn decode_commit() {
    let contents = File::open(&Path::new("resources/tests/commit")).read_to_end().unwrap();
    let decoded = commit::decode(contents.as_slice()).unwrap();

    assert_eq!(create_test_commit(), decoded);
}