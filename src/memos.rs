use crate::common::*;
use crate::Memo;


#[derive(Serialize, Deserialize, Default)]
pub struct Memos{
    next_id: u16,
    free_idxs: HashSet<u16>,
    memos: BTreeMap<u16, Memo>,
}

impl Memos{
    pub fn new() -> Self { Memos{next_id: 1, ..Default::default() } }

    pub fn push(&mut self, mut memo: Memo)->&mut Memo{
            memo.parse_tags();
            self.next_id += 1;
            self.memos.entry(memo.id()).or_insert(memo)
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

    pub fn iter_mut(&mut self)->impl Iterator<Item = &mut Memo> + '_{
        self.memos.values_mut()
    }

    pub fn last_id(&self)->u16{
        self.next_id
    }
}

