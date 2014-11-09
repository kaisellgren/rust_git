//! Meta information about Git objects.

use object_id::ObjectId;
use object_header::ObjectHeader;

#[deriving(PartialEq, Show, Clone)]
pub struct Meta {
    pub id: ObjectId,
    pub header: ObjectHeader,
}
