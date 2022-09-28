use crate::common::*;
use crate::{DocId, DocKind, ListId};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DocDescriptor {
    pub id: DocId,
    pub name: String, // name as in filesystem
    pub list: ListId,
    pub kind: DocKind,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DocMetadata {
    pub tags: HashSet<String>,
    // author ?
    // time created ?
    // time edited ?
    // time accessed ?
    // times viewed ?
}

impl DocMetadata {
    pub fn new() -> Self {
        todo!()
    }
}
