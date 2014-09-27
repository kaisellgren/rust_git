use std::fmt;

pub enum GitError {
    CorruptedRepository(&'static str),
    CorruptedCommit(&'static str)
}

impl fmt::Show for GitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CorruptedCommit(s) => write!(f, "CorruptedCommit: {}", s),
            CorruptedRepository(s) => write!(f, "CorruptedRepository: {}", s),
        }
    }
}