use crate::common::*;
use crate::{DocId, ListId};

/// per-list document id, e.g. in `note#3` 3 is a ListDocId
pub type ListDocId = u32;

pub struct List {
    path: PathBuf,
    docs: BTreeMap<ListDocId, DocId>,

    // doc ids by their names in this list
    names: BTreeMap<String, DocId>, // decide whether to use DocId or ListDocId
    // lists: BTreeSet<ListId>,
    next_list_doc_id: ListDocId,
}

impl List {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            docs: Default::default(),
            names: Default::default(),
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

    // returns Some(DocId) by ListDocId, returns None if missing
    pub fn get_by_id(&self, id: ListDocId) -> Option<DocId> {
        self.docs.get(&id).copied()
    }

    // returns Some(DocId) by name, returns None if missing
    pub fn get_by_name(&self, name: &str) -> Option<DocId> {
        self.names.get(name).copied()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
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
