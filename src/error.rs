//! Every rust_git error.
use std::io;
use serialize::hex;
use std::error::FromError;
use std::error::Error;
use std::borrow::Cow;

pub type StringCow = Cow<'static, String, str>;

#[deriving(Show)]
pub enum GitError {
    CorruptRepository(StringCow),
    CorruptCommit(StringCow),
    CorruptObject(StringCow),
    NotFound,
    InvalidString,
    HexError(hex::FromHexError),
    IoError(io::IoError),
}

impl FromError<io::IoError> for GitError {
    fn from_error(error: io::IoError) -> GitError {
        GitError::IoError(error)
    }
}

impl FromError<hex::FromHexError> for GitError {
    fn from_error(error: hex::FromHexError) -> GitError {
        GitError::HexError(error)
    }
}

impl Error for GitError {
    fn description(&self) -> &str {
        match *self {
            GitError::CorruptRepository(..) => "Corrupt repository",
            GitError::CorruptCommit(..)     => "Corrupt commit",
            GitError::CorruptObject(..)     => "Corrupt object",
            GitError::NotFound              => "Not found",
            GitError::InvalidString         => "Invalid string",
            GitError::HexError(..)          => "Invalid hex format",
            GitError::IoError(..)           => "Encountered an I/O error",
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            GitError::CorruptRepository(ref s) |
                GitError::CorruptCommit(ref s) |
                GitError::CorruptObject(ref s) => Some(s.clone().into_string()),
            _ => None,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitError::IoError(ref e) => Some(e as &Error),
            GitError::HexError(ref e) => Some(e as &Error),
            _ => None,
        }
    }
}