use std::{collections::BTreeMap, path::Path};

use crate::common::*;
use crate::{Doc, DocDescriptor, ListId};

pub type DocId = u32;

#[derive(Default)]
pub struct Docs {
    doc_descriptor_by_doc_id: BTreeMap<DocId, DocDescriptor>,
    doc_by_doc_id: BTreeMap<DocId, Doc>,

    next_doc_id: DocId,
}

impl Docs {
    // creates a new empty document without indexing it (?)
    pub fn create(&mut self) -> &mut Doc {
        let id = self.next_doc_id;
        self.next_doc_id += 1;
        // I use this trick to return a mutable reference to the document.
        self.doc_by_doc_id.entry(id).or_default()
    }

    pub fn index(&mut self, descriptor: DocDescriptor) {
        let DocDescriptor {
            id,
            name,
            list,
            kind,
            tags,
        } = descriptor.clone();

        self.doc_descriptor_by_doc_id.insert(id, descriptor);

        todo!()
    }

    pub fn write(&self, id: DocId) -> Result<()> {
        todo!();
    }

    pub fn load(path: &Path) -> Self {
        todo!()
    }

    pub fn get_docs(&self) -> impl Iterator<Item = &Doc> {
        self.doc_by_doc_id.values()
    }

    pub fn get_doc(&self, id: DocId) -> Option<&Doc> {
        self.doc_by_doc_id.get(&id)
    }

    pub fn get_doc_mut(&mut self, id: DocId) -> Option<&mut Doc> {
        self.doc_by_doc_id.get_mut(&id)
    }

    // pub fn get_doc_by_path(&self, path: &Path) -> Option<&Doc> {
    //     let id = self.doc_id_by_list_id.get(path)?;
    //     self.doc_by_doc_id.get(&id)
    // }

    // pub fn get_doc_by_path_mut(&mut self, path: &Path) -> Option<&mut Doc> {
    //     let id = self.doc_id_by_list_id.get(path)?;
    //     self.doc_by_doc_id.get_mut(&id)
    // }
}
