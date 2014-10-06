use std::fmt;

#[deriving(Show)]
pub enum GitError {
    CorruptRepository(&'static str),
    CorruptCommit(&'static str),
    NotFound,
}