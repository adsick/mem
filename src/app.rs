use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

use crate::common::*;
use crate::Memo;
use crate::memos::Memos;
pub struct MemApp{
    data: File,
    memos: Memos,
}

impl MemApp{
    pub fn init(filename: &str)->Self{
        let data = OpenOptions::new().create(true).read(true).write(true).open(filename).unwrap();
        let memos =
        if data.metadata().unwrap().len() > 0{
            from_reader(&data).unwrap()//.unwrap_or_default();
        } else {
            Memos::default()
        };
        MemApp{data, memos}
    }

    pub fn save(&mut self)->std::io::Result<()>{
        self.clear()?;
        self.data.write_all(to_string_pretty(&self.memos).unwrap().as_bytes())
    }

    pub fn clear(&mut self)->std::io::Result<()>{
        self.data.seek(SeekFrom::Start(0)).unwrap();
        self.data.set_len(0)
    }

    pub fn create_new_memo_interactive(&mut self)->&mut Memo{
        let mut memo = self.memos.push(None);

        memo.header = Text::new("Header").with_default(&format!("memo#{}", memo.id())).prompt().unwrap_or_default();
        memo.body = Text::new("Body").prompt().unwrap_or_default();
        memo
    }

    pub fn list(&self){
        for memo in self.memos.iter(){
            println!("{}\n", memo.to_string())
        }
    }
    pub fn preview_list(&self){
        for memo in self.memos.iter(){
            println!("{}", memo.preview())
        }
    }
    // pub fn edit_memo(&mut self, id: u16){


    //     Text::new("Edit").with_initial_value();
    //     todo!()
    // }
}

impl From<File> for MemApp{
    fn from(file: File) -> Self {
        let memos = from_reader(&file).unwrap();
        Self{memos, data: file}
    }
}