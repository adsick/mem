use crate::common::*;

use crate::List;

pub type ListId = u32;

#[derive(Default)]
pub struct Lists {
    // now I'm not sure about lists and their kinds
    // list_id_by_kind: HashMap<ListKind, ListId>,
    list_id_by_path: HashMap<String, ListId>,

    list_by_list_id: HashMap<ListId, List>,

    next_list_id: ListId,
}

impl Lists {
    pub fn create_list(&mut self, path: PathBuf) -> ListId {
        // self.list_id_by_kind.try_insert(path, self.next_list_id)
        let list_id = self.next_list_id;

        self.list_by_list_id
            .insert(self.next_list_id, List::new(path.clone()));

        // todo: decide how to store list paths and maybe write a normalization method for it.

        self.list_id_by_path
            .insert(path.to_string_lossy().to_string(), list_id);

        self.next_list_id += 1;
        list_id
    }

    pub fn create_list_if_not_exists(&mut self, path: PathBuf) -> Option<ListId> {
        if !self
            .list_id_by_path
            .contains_key(&path.to_string_lossy().to_string())
        {
            let id = self.create_list(path.clone());
            println!("created a new list, path = {:?}, id = {}", path, id);
            return Some(id);
        }
        None
    }
}
