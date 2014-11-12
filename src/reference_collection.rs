use reference::Reference;

#[deriving(PartialEq, Show, Clone)]
pub struct ReferenceCollection {
    pub head: Option<Reference>,
    pub local_references: Vec<Reference>,
    pub remote_references: Vec<Reference>,
}

impl ReferenceCollection {
    fn references(&self) -> Vec<Reference> {
        let mut list = self.local_references.clone();
        list.push_all(self.remote_references.as_slice());
        list
    }
}