#[deriving(PartialEq, Show)]
pub enum TagType {
    Lightweight,
    Annotated,
    Signed,
}