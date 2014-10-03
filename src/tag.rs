use meta::Meta;
use tag_type::TagType;
use object_id::ObjectId;

#[deriving(PartialEq, Show)]
pub struct Tag {
    pub meta: Meta,
    pub name: String,
    pub message: Option<String>,
    pub tagger_name: Option<String>,
    pub tagger_email: Option<String>,
    pub tag_date: Option<uint>,
    pub tag_type: TagType,
    pub target_identifier: ObjectId,
}