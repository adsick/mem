use crate::common::*;

#[derive(Default, Serialize, Deserialize)]
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

    pub fn id(&self)->u16{
        self.id
    }

    pub fn add_tag(&mut self, tag: String)->bool{
        self.tags.insert(tag)
    }

    pub fn add_link(&mut self, link: String)->bool{
        todo!()
    }

    pub fn preview(&self) -> String{
        format!("#{} {}", self.id, self.header)
    }
}

impl ToString for Memo{
    fn to_string(&self) -> String {
        let Self{id, header, topic, body, ..} = self;
        // let mut t = &String::new();
        // if let Some(topic) = topic{
        //     t = topic;

        // }
        format!("#{} {}\ntopic: {}\n{}", self.id, header, topic, body)
    }
}