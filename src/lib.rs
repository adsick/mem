mod common;
use common::*;

pub mod app;
pub use app::*;

pub mod command;
pub use command::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Memo{
    id: u16,
    pub header: String,
    pub topic: String,
    pub body: String,
    pub notes: String,
    tags: HashSet<String>,
    links: HashSet<u16>,
    //datetime: DateTime<TimeZone::>
}


impl Memo{
    pub fn new(id: u16)->Memo{
        Memo{id, ..Default::default()}
    }

    pub fn id(&self)->String{
        encode(self.id.to_le_bytes())
    }

    pub fn add_tag(&mut self, tag: String)->bool{
        self.tags.insert(tag)
    }

    pub fn add_link(&mut self, link: String)->bool{
        todo!()
    }
}

impl ToString for Memo{
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}