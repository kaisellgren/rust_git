//! Every rust_git error.
use std::io;
use std::error::FromError;
use std::error::Error;

#[deriving(Show)]
pub enum GitError {
    CorruptRepository(&'static str),
    CorruptCommit(&'static str),
    NotFound,
    IoError(io::IoError),
}

impl FromError<io::IoError> for GitError {
    fn from_error(error: io::IoError) -> GitError {
        GitError::IoError(error)
    }
}

impl Error for GitError {
    fn description(&self) -> &'static str {
        match *self {
            GitError::CorruptRepository(s) => "corrupt repository",
            GitError::CorruptCommit(s) => "corrupt commit",
            GitError::NotFound => "not found",
            GitError::IoError(ref e) => "encountered an I/O error",
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            GitError::CorruptRepository(s) => Some(s.into_string()),
            GitError::CorruptCommit(s) => Some(s.into_string()),
            _ => None,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitError::IoError(ref e) => Some(e as &Error),
            _ => None,
        }
    }
}