use crate::{DocDescriptor, ListDocId, ListKind};
pub use crate::{DocId, List};
pub use std::collections::HashMap;
use std::path::PathBuf;

pub type ListId = u32;

#[derive(Default)]
pub struct Index {
    // not sure if this is a good solution to finding files
    doc_path_by_doc_id: HashMap<DocId, PathBuf>,
    doc_id_by_doc_path: HashMap<PathBuf, DocId>,

    list_id_by_kind: HashMap<ListKind, ListId>,
    list_by_list_id: HashMap<ListId, List>,
    next_list_id: ListId,
}

impl Index {
    // returns builder-like index thing
    // example: index.get(5).kind(ListKind::Note)
    pub fn get(&self, id: ListDocId) -> IndexOp {
        IndexOp::new(self, id)
    }

    pub fn index(&mut self, descriptor: DocDescriptor) {
        let DocDescriptor {
            id,
            path,
            kind,
            tags,
        } = descriptor;
    }

    pub fn create_list(&mut self, kind: ListKind) -> ListId {
        // self.list_id_by_kind.try_insert(path, self.next_list_id)
        let list_id = self.next_list_id;

        self.list_by_list_id
            .insert(self.next_list_id, List::new(kind));
        self.next_list_id += 1;
        list_id
    }
}

pub struct IndexOp<'i> {
    index: &'i Index,
    list_doc_id: ListDocId,
}

impl<'i> IndexOp<'i> {
    fn new(index: &'i Index, list_doc_id: ListDocId) -> Self {
        Self { index, list_doc_id }
    }
    //todo: proper error messages (like "list not found")
    // get doc id by list id
    fn id(&self, id: ListId) -> Option<DocId> {
        self.index
            .list_by_list_id
            .get(&id)
            .and_then(|list| list.get(self.list_doc_id))
    }

    // get doc id using list kind
    fn kind(&self, kind: &ListKind) -> Option<DocId> {
        let Index {
            list_id_by_kind,
            list_by_list_id,
            ..
        } = self.index;
        let list_id = list_id_by_kind.get(kind)?;
        list_by_list_id.get(list_id)?.get(self.list_doc_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let index = Index::default();
        assert_eq!(index.get(0).id(0), None);
    }
}
