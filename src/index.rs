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
    // and then make relative paths for contained lists

    /// traverses given directory in order to fetch some data from it.
    pub fn scan(&mut self, mut path: &Path) -> Result<()> {
        // check path long way (using .metadata()) to get errors (e.g. missing)
        if path.metadata()?.is_dir() {
            
            // convert abs path to relative, note this is not working (produces empty path, need to fix)
            // a more dumb but working solution would be
            // to strip prefix for every entry (in scan_recursive), but that is not so efficient imo
            if path.is_absolute(){
                let root_path = self.lists.root().path();
                println!("trying to strip prefix {root_path:?} from {path:?}");
                path = path.strip_prefix(root_path)?;
                
                println!("converted absolute path to relative: {:?}", path);
                // consider crate `relative_path`
            }


            self.scan_recursive(path, 0); // the list tree begins from 0 id
            Ok(())
        } else {
            Err(Error::msg(format!("malformed path: {:?}", path)))
        }
    }

    // path is the path that we traverse now
    // list_id points the list we are currently in
    fn scan_recursive(&mut self, path: &Path, list_id: ListId) -> Result<()> {
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("dir: {:?}", &path);

                // create list if it does not exist
                // need to come up with a method of providing relative path.
                let (Ok(id) | Err(id)) = self.lists.create_if_not_exists(path.clone()); // into_ok_or_err (unstable, https://doc.rust-lang.org/std/result/enum.Result.html#method.into_ok_or_err)

                self.scan_recursive(&path, id);
            } else if path.is_file() {




                // // check if this file is already known
                // if let Some(id) = self.docs.get_doc_by_path(&path) {
                //     // file is already known
                // } else {
                //     // file is not known
                // }
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
        let mut index = Index::new(mem_dir);
        // assert_eq!(index.get(0).id(0), None);

        index.scan(&PathBuf::from_str(".").unwrap());
    }
}
