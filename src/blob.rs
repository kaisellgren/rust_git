use meta::Meta;

#[deriving(PartialEq, Show)]
pub struct Blob {
    pub meta: Meta,
    pub size: uint,
    pub contents: Vec<u8>,
}