use super::*;

pub struct IndexOp<'i> {
    index: &'i Index, // todo: take ref to lists maybe
    list_doc_id: ListDocId,
}

impl<'i> IndexOp<'i> {
    pub fn new(index: &'i Index, list_doc_id: ListDocId) -> Self {
        Self { index, list_doc_id }
    }
    //todo: proper error messages (like "list not found")
    // get doc id by list id
    pub fn id(&self, id: ListId) -> Option<DocId> {
        self.index
            .list_by_list_id
            .get(&id)
            .and_then(|list| list.get(self.list_doc_id))
    }

    // // get doc id using list kind
    // pub fn kind(&self, kind: &ListKind) -> Option<DocId> {
    //     let Index {
    //         list_id_by_kind,
    //         list_by_list_id,
    //         ..
    //     } = self.index;
    //     let list_id = list_id_by_kind.get(kind)?;
    //     list_by_list_id.get(list_id)?.get(self.list_doc_id)
    // }
}
