use std::{collections::BTreeMap, path::Path};

use crate::{Doc, Index};

pub type DocId = u32;

#[derive(Default)]
pub struct Docs {
    cache: BTreeMap<DocId, Doc>,
    next_doc_id: DocId,
    index: Index,
}

impl Docs {
    pub fn add_new(&mut self, doc: Doc) -> DocId {
        todo!()
    }

    pub fn load(path: &Path) -> Self{
        todo!()
    }
}
