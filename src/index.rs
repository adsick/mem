use crate::common::*;
use crate::{DocDescriptor, ListDocId, ListKind, Tag};

pub mod index_op;
pub use index_op::*;

pub use crate::{DocId, List};
pub use std::collections::HashMap;
use std::path::PathBuf;

pub type ListId = u32;

#[derive(Default)]
pub struct Index {
    // not sure if this is a good solution to finding files
    doc_id_by_doc_path: HashMap<PathBuf, DocId>,

    doc_descriptor_by_doc_id: BTreeMap<DocId, DocDescriptor>,

    // now I'm not sure about lists and their kinds
    list_id_by_kind: HashMap<ListKind, ListId>,
    list_id_by_path: HashMap<String, ListId>,

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
        } = descriptor.clone();

        self.doc_id_by_doc_path.insert(path.clone(), id);
        self.doc_descriptor_by_doc_id.insert(id, descriptor);

        todo!()
    }

    pub fn create_list(&mut self, kind: ListKind) -> ListId {
        // self.list_id_by_kind.try_insert(path, self.next_list_id)
        let list_id = self.next_list_id;

        self.list_by_list_id
            .insert(self.next_list_id, List::new(kind));
        self.next_list_id += 1;
        list_id
    }

    // todo: decide whether to use absolute or relative paths.
    // I personally want relative, but not sure if it gonna work.
    /// traverses given directory in order to fetch some data from it.
    pub fn scan(&mut self, path: &Path) -> Result<()> {
        for entry in path.read_dir()?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {

                let mut path_str = path.to_string_lossy().to_string();

                dbg!(&path_str);

                // this is a bit cringe
                // let mut path_str = path_str.strip_prefix("./").unwrap_or(path_str);
                // path_str = path_str.strip_suffix("/").unwrap();
                // dbg!(&path_str);
                

                self.list_id_by_path.get(&path_str);

                self.scan(&path);
            } else if path.is_file() {
                // check if this file is already known
                if let Some(id) = self.doc_id_by_doc_path.get(&path) {
                } else {
                    // now need to check
                    // todo!()
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut index = Index::default();
        assert_eq!(index.get(0).id(0), None);

        index.scan(&document_dir().unwrap());

    }
}
