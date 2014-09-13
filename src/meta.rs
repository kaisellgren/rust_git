use object_id::ObjectId;
use object_header::ObjectHeader;

pub struct Meta {
    pub id: ObjectId,
    pub header: ObjectHeader,
}
