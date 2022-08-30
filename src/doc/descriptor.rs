use crate::common::*;
use crate::{ListId, DocId, DocKind};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct DocDescriptor {
    pub id: DocId,
    pub name: String,
    pub list: ListId,
    pub kind: DocKind,
    //meta
    pub tags: HashSet<String>,
}
