use crate::common::*;

pub struct ListDescriptor {
    path: PathBuf,
    kind: ListKind,
}

impl ListDescriptor {
    pub fn new(path: PathBuf) -> Self {
        ListDescriptor {
            path,
            kind: ListKind::Doc,
        }
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
    // User
}
