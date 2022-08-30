use crate::common::*;

use crate::List;

pub type ListId = u32;

pub struct Lists {
    // maybe refactor it to be in Index
    root: List,
    // now I'm not sure about lists and their kinds
    // list_id_by_kind: HashMap<ListKind, ListId>,
    list_id_by_path: HashMap<String, ListId>,

    list_by_list_id: HashMap<ListId, List>,

    next_list_id: ListId,
}

impl Lists {
    pub fn new(path: PathBuf) -> Self {
        Self {
            root: List::new(path),
            list_by_list_id: Default::default(),
            list_id_by_path: Default::default(),
            next_list_id: Default::default(),
        }
    }

    pub fn create_relative(&mut self, path: PathBuf) -> ListId {
        let list_id = self.next_list_id;

        self.list_by_list_id
            .insert(self.next_list_id, List::new(path.clone()));

        // todo: decide how to store list paths and maybe write a normalization method for it.

        self.list_id_by_path
            .insert(path.to_string_lossy().to_string(), list_id);

        self.next_list_id += 1;
        list_id
    }

    // todo: rename

    // creates a list if there is no such, returns it's id in Ok variant,
    // if there was list returns Err(id)
    pub fn create_if_not_exists(&mut self, relative_path: PathBuf) -> Result<ListId, ListId> {
        // you should not call this method with root path
        if self.root.path() == &relative_path {
            return Err(0);
        }
        // assert!(!relative_path.starts_with(self.root.path()), "method 'create_if_not_exists was called with a path that is not inside the root directory, abort");
        let path_str = relative_path.to_string_lossy().to_string();

        if let Some(id) = self.list_id_by_path.get(&path_str) {
            // case where list already exists - we just return it's id
            Err(*id)
        } else {
            // the list does not exist, create it
            let id = self.create_relative(relative_path.clone()); // todo: double check relativity
            println!("created a new list, path = {:?}, id = {}", relative_path, id);
            Ok(id)
        }
    }

    pub fn root(&self) -> &List {
        &self.root
    }
}
