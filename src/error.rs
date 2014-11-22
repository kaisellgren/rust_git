//! Every rust_git error.

#[deriving(Show)]
pub enum GitError {
    CorruptRepository(&'static str),
    CorruptCommit(&'static str),
    NotFound,
}