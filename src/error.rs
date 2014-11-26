//! Every rust_git error.
use std::io;
use std::error::FromError;
use std::error::Error;
use std::str::MaybeOwned;

#[deriving(Show)]
pub enum GitError {
    CorruptRepository(MaybeOwned<'static>),
    CorruptCommit(MaybeOwned<'static>),
    CorruptObject(MaybeOwned<'static>),
    NotFound,
    IoError(io::IoError),
}

impl FromError<io::IoError> for GitError {
    fn from_error(error: io::IoError) -> GitError {
        GitError::IoError(error)
    }
}

impl Error for GitError {
    fn description(&self) -> &str {
        match *self {
            GitError::CorruptRepository(..) => "corrupt repository",
            GitError::CorruptCommit(..)     => "corrupt commit",
            GitError::CorruptObject(..)     => "corrupt object",
            GitError::NotFound             => "not found",
            GitError::IoError(..)       => "encountered an I/O error",
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            GitError::CorruptRepository(ref s) => Some(s.clone().into_string()),
            GitError::CorruptCommit(ref s)     => Some(s.clone().into_string()),
            GitError::CorruptObject(ref s)     => Some(s.clone().into_string()),
            _                                  => None,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitError::IoError(ref e) => Some(e as &Error),
            _ => None,
        }
    }
}