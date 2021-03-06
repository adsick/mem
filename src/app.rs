use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

use inquire::error::InquireError;

use crate::common::*;
use crate::Memo;
use crate::memos::Memos;

use crate::search::{QueryResults, SearchQueryKind};
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
            Memos::new()
        };
        MemApp{data, memos}
    }

    pub fn get_memo(&self, id: u16)->Option<&Memo>{
        self.memos.get(id)
    }

    pub fn get_memo_mut(&mut self, id: u16)->Option<&mut Memo>{
        self.memos.get_mut(id)
    }

    pub fn edit_interactive(&mut self, id: u16){
        if let Some(memo) = self.memos.get_mut(id){
            let actions = vec![EditAction::Add, EditAction::Remove, EditAction::Edit];

            let action =
            Select::new("action: ", actions).prompt().unwrap();

            fn input(message: &str)->Result<String, InquireError>{
                Text::new(message).prompt()
            }
            fn edit(field: &mut String){
                if let Ok(new_field) = Text::new("edit").with_initial_value(field).prompt(){
                    *field = new_field 
                }
            }
            let fields = vec![MemoField::Header, MemoField::Topic, MemoField::Body, MemoField::Tag];
            match Select::new("field: ", fields).prompt().unwrap(){
                MemoField::Header => {
                    match action{
                        EditAction::Add => todo!(),
                        EditAction::Remove => todo!(),
                        EditAction::Edit => edit(&mut memo.header),
                    }
                },
                MemoField::Topic => {
                    match action{
                        EditAction::Add => todo!(),
                        EditAction::Remove => todo!(),
                        EditAction::Edit => edit(&mut memo.topic),
                    }
                },
                MemoField::Body => {
                    match action{
                        EditAction::Add => todo!(),
                        EditAction::Remove => if let Ok(true) = Confirm::new("remove body?").prompt(){memo.body.clear()}, //note remove tags
                        EditAction::Edit => {edit(&mut memo.body); memo.parse_tags()},
                    }
                },
                MemoField::Tag => {
                    match action{
                        EditAction::Add => while let Ok(tag ) = input("add tag"){ if tag.is_empty(){break;} memo.add_tag(tag);},
                        EditAction::Remove => memo.tags.clear(),
                        EditAction::Edit => todo!(),
                    }
                },
                MemoField::Link => {
                    todo!()
                },
            }

        } else {
            println!("memo number #{} does not exist.\n", id)
        }

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
        let mut memo = Memo::new(self.last_id());
        memo.header = Text::new("Header").with_default(&format!("memo#{}", memo.id())).prompt().unwrap_or_default();
        memo.body = Text::new("Body").prompt().unwrap_or_default();
        self.memos.push(memo)
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

    pub fn execute_query<'m>(&'m self, query_kind: SearchQueryKind<'m>)->impl Iterator<Item = &Memo> + 'm{
        match query_kind{
            SearchQueryKind::Memo(query) =>self.memos.iter().filter_map(move |m|m.select(query.clone())) // TODO Remove .clone(),
        }
 
    }

    pub fn query<'m>(&'m self, query: SearchQueryKind<'m>)->QueryResults<'m>{
        QueryResults::from(self.execute_query(query))
    }

    pub fn search<'a>(&'a self, query: SearchQueryKind<'a>){
        for memo in self.execute_query(query){
            println!("{}", memo.preview())
        }
    }

    pub fn last_id(&self)->u16{
        self.memos.last_id()
    }
}

impl From<File> for MemApp{
    fn from(file: File) -> Self {
        let memos = from_reader(&file).unwrap();
        Self{memos, data: file}
    }
}


enum EditAction{
    Add,
    Remove,
    Edit
}

impl std::fmt::Display for EditAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self{
            EditAction::Add => "add",
            EditAction::Remove => "remove",
            EditAction::Edit => "edit",
        };
        write!(f, "{}", res)
    }
}


enum MemoField{
    Header,
    Topic,
    Body,
    Tag,
    Link,
}

impl std::fmt::Display for MemoField{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self{
            MemoField::Header => "header",
            MemoField::Topic => "topic",
            MemoField::Body => "body",
            MemoField::Tag => "tag",
            MemoField::Link => "link",
        };
        write!(f, "{}", res)
    }
}