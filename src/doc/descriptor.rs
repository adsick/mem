use crate::common::*;
use crate::{DocId, DocKind};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct DocDescriptor {
    pub id: DocId,
    pub path: PathBuf,
    pub kind: DocKind,
    //meta
    pub tags: HashSet<String>,
}
