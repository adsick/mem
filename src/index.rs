use crate::common::*;
use crate::{Doc, DocDescriptor, ListDocId, ListKind, Tag};

// temporarily commented to ignore errors
// pub mod index_op;
// pub use index_op::*;

pub mod docs;
pub use docs::*;

pub mod lists;
pub use lists::*;

pub use crate::List;
pub use std::collections::HashMap;
use std::path::PathBuf;

pub type ListId = u32;

// it can be logically refactored to Index { Docs, Lists}

#[derive(Default)]
pub struct Index {
    docs: Docs,
    lists: Lists,
}

impl Index {
    // returns builder-like index thing
    // example: index.get(5).kind(ListKind::Note)
    // pub fn get(&self, id: ListDocId) -> IndexOp {
    //     IndexOp::new(self, id)
    // }

    // todo: decide whether to use absolute or relative paths.
    // I personally want relative, but not sure if it gonna work.
    /// traverses given directory in order to fetch some data from it.
    pub fn scan(&mut self, path: &Path) -> Result<()> {
        // check path long way to get errors (e.g. missing)
        if path.metadata()?.is_dir() {
            self.lists.create_list_if_not_exists(path.to_owned());

            self.scan_recursive(path);
            Ok(())
        } else {
            Err(Error::msg(format!("malformed path: {:?}", path)))
        }
    }

    fn scan_recursive(&mut self, path: &Path) -> Result<()> {
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("dir: {:?}", &path);

                // this is a bit cringe
                // let mut path_str = path_str.strip_prefix("./").unwrap_or(path_str);
                // path_str = path_str.strip_suffix("/").unwrap();
                // dbg!(&path_str);

                // create list if it does not exist
                self.lists.create_list_if_not_exists(path.clone());

                self.scan(&path);
            } else if path.is_file() {
                // check if this file is already known
                if let Some(id) = self.docs.get_doc_by_path(&path) {
                    // file already known
                } else {
                    // ~~now need to create all the missing paths and create them~~ no need coz we've done it in the previous if branch
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
        // assert_eq!(index.get(0).id(0), None);

        index.scan(&PathBuf::from_str(".").unwrap());
    }
}
