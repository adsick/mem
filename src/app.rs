use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::common::*;
use crate::Memo;
pub struct MemApp{
    data: File,
    memos: HashMap<u16, Memo>,
}

impl MemApp{
    pub fn init(filename: &str)->Self{
        let data = OpenOptions::new().create(true).read(true).write(true).open(filename).unwrap();
        let memos = from_reader(&data).unwrap_or_default();
        MemApp{data, memos}
    }

    pub fn save(&mut self)->std::io::Result<()>{
        self.data.write_all(to_string_pretty(&self.memos).unwrap().as_bytes())
    }

    pub fn add_memo(&mut self, memo: Option<Memo>)->&mut Memo{
        let mut id = random();
        while self.memos.contains_key(&id){
            id = random();
        }
        let memo = self.memos.entry(id).or_insert(memo.unwrap_or_else(||Memo::new(id)));
        memo
    }

    pub fn get_memo(&self, id: u16)->Option<&Memo>{
        self.memos.get(&id)
    }

    pub fn get_memo_mut(&mut self, id: u16)->Option<&mut Memo>{
        self.memos.get_mut(&id)
    }

    pub fn list(&self)->impl Iterator<Item = String> + '_{
        self.memos.values().map(|m|{
            m.to_string()
        })
    }
}

impl From<File> for MemApp{
    fn from(file: File) -> Self {
        let memos = from_reader(&file).unwrap();
        Self{memos, data: file}
    }
}