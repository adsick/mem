use crate::common::*;
use crate::Memo;


#[derive(Serialize, Deserialize, Default)]
pub struct Memos{
    last_id: u16,
    free_idxs: HashSet<u16>,
    memos: BTreeMap<u16, Memo>,
}

impl Memos{
    pub fn new() -> Self { Self::default() }

    pub fn push(&mut self, memo: Option<Memo>)->&mut Memo{
        self.last_id += 1;
        let id = self.last_id;
        let memo = self.memos.entry(id).or_insert(memo.unwrap_or_else(||Memo::new(id)));
        memo
    }

    pub fn get(&self, id: u16)->Option<&Memo>{
        self.memos.get(&id)
    }

    pub fn get_mut(&mut self, id: u16)->Option<&mut Memo>{
        self.memos.get_mut(&id)
    }

    pub fn iter(&self)->impl Iterator<Item = &Memo> + '_{
        self.memos.values()
    }

    pub fn last_id(&self)->u16{
        self.last_id
    }
}

