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
            next_list_id: 1,
        }
    }

    pub fn create_relative(&mut self, path: PathBuf) -> ListId {
        let list_id = self.next_list_id;

        self.list_by_list_id
            .insert(self.next_list_id, List::new(path.clone()));

        self.list_id_by_path
            .insert(path.to_string_lossy().to_string(), list_id);

        println!("created a new list, path = {path:?}, id = {}", list_id);

        self.next_list_id += 1;
        list_id
    }

    // todo: rename, maybe change to accept String, not PathBuf

    // creates a list if there is no such, returns it's id in Ok variant,
    // if there was list returns Err(id)
    pub fn create_if_not_exists<'a>(
        &mut self,
        path: impl Into<Cow<'a, Path>>,
    ) -> Result<ListId, ListId> {
        let path = path.into();
        assert!(path.is_relative());

        let path_str = path.to_string_lossy();

        if let Some(id) = self.list_id_by_path.get(path_str.as_ref()) {
            // case where list already exists - we just return it's id
            Err(*id)
        } else {
            // the list does not exist, create it
            let id = self.create_relative(path.into_owned());
            Ok(id)
        }
    }

    pub fn root(&self) -> &List {
        &self.root
    }

    pub fn get_list_by_id(&self, id: ListId) -> Option<&List> {
        self.list_by_list_id.get(&id)
    }
}
