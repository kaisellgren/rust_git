use object_id::ObjectId;

pub struct Branch {
    pub name: String,
    pub canonical_name: String,
    pub tip_id: ObjectId,
    pub tracked_branch: Option<Box<Branch>>,
    pub is_remote: bool,
    pub is_detached: bool,
}

impl Branch {
    pub fn is_tracking(&self) -> bool {
        self.tracked_branch.is_some()
    }
}