use std::fmt;

pub enum GitError {
    CorruptRepository(&'static str),
    CorruptCommit(&'static str),
    NotFound,
}

impl fmt::Show for GitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CorruptCommit(s) => write!(f, "CorruptCommit: {}", s),
            CorruptRepository(s) => write!(f, "CorruptRepository: {}", s),
            NotFound => write!(f, "NotFound"),
        }
    }
}