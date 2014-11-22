use commit::Commit;
use blob::Blob;
use tag::Tag;
//use tree::Tree;
//use note::Note;

pub enum GitObject {
    GitCommit(Commit),
    GitBlob(Blob),
    GitTag(Tag),
    GitTree,
    GitNote,
}