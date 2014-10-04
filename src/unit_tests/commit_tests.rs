use std::io::File;
use commit::Commit;
use commit;
use eobject::EObject;
use object_type;
use object_header::ObjectHeader;
use object_id::ObjectId;
use meta::Meta;
use serialization::Serializable;

use object_database;

fn create_test_commit() -> Commit {
    Commit {
        meta: Meta {
            id: ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741140"),
            header: ObjectHeader {
                typ: object_type::Commit,
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
        parent_ids: vec![ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741141"), ObjectId::from_string("b744d5cddb5095249299d95ee531cbd990741142")]
    }
}

#[test]
fn encode_commit() {
    let expected = File::open(&Path::new("resources/tests/commit")).read_to_end().unwrap();
    let encoded = create_test_commit().encode();

    assert_eq!(expected, encoded);
}

#[test]
fn decode_commit() {
    let contents = File::open(&Path::new("resources/tests/commit")).read_to_end().unwrap();
    let decoded = commit::decode(contents.as_slice()).unwrap();

    assert_eq!(create_test_commit(), decoded);
}

#[test]
fn foo() {
    /*let repo = Repository {
        path: "./"
    };*/

    let id = ObjectId::from_string("0b9401c1e9595279b7da5335739e6648a3320203");
    let res = *object_database::find_object_by_id(&id).unwrap();
    match res {
        ECommit(c) => println!("{}", c.meta.id),
        _ => println!("dang"),
    }
    //println!("{}", res.meta.id);

    fail!("STOP");
}