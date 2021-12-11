use chrono::Local;

use crate::common::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Memo{
    id: u16,
    pub header: String,
    pub topic: String,
    pub body: String,
    //pub notes: String,
    pub tags: HashSet<String>,
    //pub links: HashSet<u16>,
    #[cfg(feature = "time")]
    datetime: Option<DateTime<Local>> //use option to derive default
}

impl Memo{
    pub fn new(id: u16)->Memo{
        let datetime = Some(Local::now());
        Memo{id, datetime, ..Default::default()}
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
        let Self{id, header, topic, body, tags, ..} = self;
        let mut res = format!("#{} {}\n", id, header);
        if !topic.is_empty(){
            res += &format!("\ttopic: {}\n", topic);
        }

        #[cfg(feature = "time")]
        if let Some(dt) = self.datetime{
            res += &format!("from: {}\n", dt.format("%F %R"))
        }

        if !body.is_empty(){
            res += &body;
            res += "\n";
        }
        for tag in tags{
            res += &format!("#{} ", tag);
        }
        //res += &tags.iter().map(|t|String::from("#") + t).collect::<Vec<_>>().join(" ");
        res
    }
}