use object_id::ObjectId;
use object_header::ObjectHeader;

#[deriving(PartialEq, Show)]
pub struct Meta {
    pub id: ObjectId,
    pub header: ObjectHeader,
}
