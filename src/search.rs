use crate::common::*;
use crate::memo::MemoSearchQuery;
use crate::{Memo, Memos};

pub struct QueryResults<'m>{
    entries: BTreeMap<u16, &'m Memo>
}

impl<'m> QueryResults<'m>{
    pub fn new(entries: BTreeMap<u16, &'m Memo>)->Self{
        Self{entries}
    }

    pub fn list_results(&self){
        for result in self.entries.iter(){
            println!("{}\n", result.1.to_string())
        }
    }
    pub fn preview_results(&self){
        for result in self.entries.iter(){
            println!("{}\n", result.1.preview())
        }
    }
    pub fn len(&self)->usize{
        self.entries.len()
    }
    pub fn is_empty(&self)->bool{
        self.entries.is_empty()
    }
}

impl<'a, T> From<T> for QueryResults<'a>
where T: Iterator<Item = &'a Memo> + 'a
{
    fn from(results: T) -> Self {
        QueryResults{entries: BTreeMap::from_iter(results.map(|m|(m.id(), m)))}
    }
}

// pub trait Search {
//     fn search(&self, query: SearchQueryKind) -> Option<&Self>;
//     fn search_mut(&mut self, query: SearchQueryKind) -> Option<&mut Self>;
// }




pub enum SearchQueryKind<'a>{
    Memo(MemoSearchQuery<'a>),
    //Todo(TodoSearchQuery)
}