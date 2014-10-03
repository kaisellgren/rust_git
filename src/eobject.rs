use commit::Commit;
use blob::Blob;
use tag::Tag;
//use tree::Tree;
//use note::Note;

pub enum EObject {
    ECommit(Commit),
    EBlob(Blob),
    ETag(Tag),
    //ETree(Tree),
    //ENote(Note),
}