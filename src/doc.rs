pub mod note;

use crate::common::*;
use crate::DocId;

use note::Note;
use serde::{Deserialize, Serialize};

use strum::EnumDiscriminants;

#[derive(Default, Deserialize, Serialize)]
pub struct DocDescriptor {
    id: DocId,
    path: PathBuf,
    kind: DocKind,
    //meta
    tags: HashSet<String>,
}

impl DocDescriptor {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

// maybe use Strum for discriminants and other stuff? (https://docs.rs/strum/latest/strum/derive.EnumDiscriminants.html)
// looks a bit like an overkill tho
#[derive(Default, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(Default, Deserialize, Serialize))]
#[strum_discriminants(name(DocKind))]
pub enum Doc {
    #[default]
    #[strum_discriminants(default)]
    Empty,

    Note(Note),
    Todo,
    Card,
    Read,
}

impl Doc {
    // consider separate new methods for each doc kind
    pub fn new() -> Self {
        Self::Empty
    }

    pub fn load(descriptor: DocDescriptor) -> Result<Self> {
        Self::try_from(descriptor)
    }

    fn note(content: &str) -> Result<Self>{
        Ok(Doc::Note(Note::from_str(&content)?))
    }
}

impl TryFrom<DocDescriptor> for Doc{
    fn try_from(descriptor: DocDescriptor) -> Result<Self> {
        let content = read_to_string(descriptor.path)?;

        let doc =
        match descriptor.kind{
            DocKind::Empty => Doc::Empty,
            DocKind::Note => Doc::note(&content)?,
            DocKind::Todo => todo!(),
            DocKind::Card => todo!(),
            DocKind::Read => todo!(),
        };

        Ok(doc)
    }

    type Error = Error;
}



pub trait Document: Display {}
