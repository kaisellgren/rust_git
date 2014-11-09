//! Represents the types of Git tags.

#[deriving(PartialEq, Show)]
pub enum TagType {
    Lightweight,
    Annotated,
    Signed,
}