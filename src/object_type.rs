pub enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
    Note,
}

impl ObjectType {
    pub fn to_text(&self) -> &'static str {
        match self {
            &Commit => "commit",
            &Tree => "tree",
            &Blob => "blob",
            &Tag => "tag",
            &Note => "note",
        }
    }

    pub fn from_text(text: &'static str) -> ObjectType {
        match text {
            "commit" => Commit,
            "tree" => Tree,
            "blob" => Blob,
            "tag" => Tag,
            "note" => Note,
            _ => fail!("Cannot convert `{}` to an ObjectType!", text)
        }
    }
}
