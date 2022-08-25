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
    // creates a new empty document without indexing it (?)
    pub fn create_new(&mut self) -> &mut Doc {
        let id = self.next_doc_id;
        self.next_doc_id += 1;
        // I use this trick to return a mutable reference to the document.
        self.cache.entry(id).or_default()
    }

    pub fn index(&mut self, id: DocId) {}

    pub fn load(path: &Path) -> Self {
        todo!()
    }

    pub fn get_cached_docs(&self) -> impl Iterator<Item = &Doc> {
        self.cache.values()
    }
}
