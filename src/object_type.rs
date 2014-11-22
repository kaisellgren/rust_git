//! Represents the types of Git objects.

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
            ObjectType::Tree => "tree",
            ObjectType::Blob => "blob",
            ObjectType::Tag => "tag",
            ObjectType::Note => "note",
        }
    }
}

pub fn from_text(text: &str) -> ObjectType {
    match text {
        "commit" => ObjectType::Commit,
        "tree" => ObjectType::Tree,
        "blob" => ObjectType::Blob,
        "tag" => ObjectType::Tag,
        "note" => ObjectType::Note,
        _ => panic!("Cannot convert `{}` to an ObjectType!", text)
    }
}