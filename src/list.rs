use std::collections::BTreeMap;
use crate::common::*;
use crate::{DocId, ListId};

/// per-list document id, e.g. in `note#3` 3 is a ListDocId
pub type ListDocId = u32;

pub struct List {
    kind: ListKind,
    docs: BTreeMap<ListDocId, DocId>,
    // lists: BTreeSet<ListId>,
    next_list_doc_id: ListDocId,
}

impl List {
    pub fn new(kind: ListKind) -> Self {
        Self {
            kind,
            docs: Default::default(),
            // lists: Default::default(),
            next_list_doc_id: 1,
        }
    }

    pub fn insert_doc(&mut self, id: DocId) -> ListDocId {
        let list_doc_id = self.next_list_doc_id;

        self.docs.insert(list_doc_id, id);

        self.next_list_doc_id += 1;
        list_doc_id
    }

    // pub fn insert_list(&mut self, id: ListId) -> bool {
    //     self.lists.insert(id)
    // }

    pub fn get(&self, id: ListDocId) -> Option<DocId> {
        self.docs.get(&id).copied()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum ListKind {
    /// universal list for all the docs
    Doc,
    /// predefined lists for different doctypes
    Note,
    Todo,
    Card,
    Read,
}
