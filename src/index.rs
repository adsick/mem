use crate::common::*;
use crate::{Doc, DocDescriptor, ListDocId, ListKind, Tag};

// temporarily commented to ignore errors
// pub mod index_op;
// pub use index_op::*;

pub mod docs;
pub use docs::*;

pub mod lists;
pub use lists::*;

use crate::List;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Index {
    docs: Docs,
    lists: Lists,
}

impl Index {
    pub fn new(path: PathBuf) -> Self {
        Index {
            docs: Docs::default(),
            lists: Lists::new(path),
        }
    }

    // returns builder-like index thing
    // example: index.get(5).kind(ListKind::Note)
    // pub fn get(&self, id: ListDocId) -> IndexOp {
    //     IndexOp::new(self, id)
    // }

    // todo: decide whether to use absolute or relative paths.
    // I personally want relative, but not sure if it gonna work.
    // upd: we call scan with an absolute path,
    // and then make relative paths for contained lists by stripping root from them
    // stripping could be avoided by using set_current_dir to root
    // and working with relative paths directly but I'm not 100% sure if it is good

    /// traverses given directory in order to fetch some data from it.
    pub fn scan(&mut self, mut path: &Path) -> Result<()> {
        // check path long way (using .metadata()) to get errors (e.g. missing)
        if path.metadata()?.is_dir() && path.is_absolute() {
            if path == self.lists.root().path() {
                self.scan_root()?;
            } else {
                unimplemented!();
                let list_id = todo!();
                self.scan_recursive(path, list_id)?;
            }
            Ok(())
        } else {
            Err(Error::msg(format!("malformed path: {:?}", path)))
        }
    }

    // special case because Lists.root is not just 0th list
    fn scan_root(&mut self) -> Result<()> {
        let path = self.lists.root().path();
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("scanning dir: {path:?}");

                let root_path = self.lists.root().path();
                let list_path = path.strip_prefix(root_path)?;

                // create list if it does not exist
                let (Ok(next_list_id) | Err(next_list_id)) =
                    self.lists.create_if_not_exists(list_path); // into_ok_or_err (unstable, https://doc.rust-lang.org/std/result/enum.Result.html#method.into_ok_or_err)

                self.scan_recursive(&path, next_list_id);
            } else if path.is_file() {
                // here we try to find this file in currently traversed list
                let filename = path.file_name().unwrap().to_string_lossy();
                if let Some(doc_id) = self.lists.root().get_by_name(&filename) {
                    // doc is known, check if it has changed (and update if has)
                    let doc = self.docs.get_doc_mut(doc_id).unwrap();
                } else {
                    // doc is unknown, need to create it
                }
            }
        }
        Ok(())
    }

    // path is the path that we traverse now
    // list_id points the list we are currently in
    fn scan_recursive(&mut self, path: &Path, list_id: ListId) -> Result<()> {
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("scanning dir: {path:?}");

                let root_path = self.lists.root().path();
                let list_path = path.strip_prefix(root_path)?;

                // create list if it does not exist
                let (Ok(next_list_id) | Err(next_list_id)) =
                    self.lists.create_if_not_exists(list_path); // into_ok_or_err (unstable, https://doc.rust-lang.org/std/result/enum.Result.html#method.into_ok_or_err)

                self.scan_recursive(&path, next_list_id);
            } else if path.is_file() {
                // another way is to simply store the root list in the map, but that is not perfect too, we might want
                // the root list to be of different type in the future.

                // here we try to find this file in currently traversed list
                let filename = path.file_name().unwrap().to_string_lossy();
                if let Some(doc_id) = self
                    .lists
                    .get_list_by_id(list_id)
                    .unwrap()
                    .get_by_name(&filename)
                {
                    // doc is known, check if it has changed (and update if has)
                    let doc = self.docs.get_doc_mut(doc_id).unwrap();
                } else {
                    // doc is unknown, need to create it
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use dirs::home_dir;

    use super::*;
    #[test]
    fn basic() {
        // todo: move tests to appropriate directory to not break your mems
        let mem_dir = home_dir().unwrap().join("mem/");

        assert!(std::env::set_current_dir(&mem_dir).is_ok());
        let mut index = Index::new(mem_dir.clone());
        // assert_eq!(index.get(0).id(0), None);

        index.scan(&mem_dir).unwrap();
    }
}
