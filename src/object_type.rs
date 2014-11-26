//! Represents the types of Git objects.
use error::GitError;
use error::GitError::CorruptObject;

#[deriving(PartialEq, Show, Clone)]
pub enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
    Note,
}

impl ObjectType {
    pub fn to_text(&self) -> &str {
        match *self {
            ObjectType::Commit => "commit",
            ObjectType::Tree   => "tree",
            ObjectType::Blob   => "blob",
            ObjectType::Tag    => "tag",
            ObjectType::Note   => "note",
        }
    }
}

pub fn from_text(text: &str) -> Result<ObjectType, GitError> {
    match text {
        "commit" => Ok(ObjectType::Commit),
        "tree"   => Ok(ObjectType::Tree),
        "blob"   => Ok(ObjectType::Blob),
        "tag"    => Ok(ObjectType::Tag),
        "note"   => Ok(ObjectType::Note),
        _        => Err(CorruptObject(format!("Cannot convert `{}` to an ObjectType!", text).into_maybe_owned()))
    }
}