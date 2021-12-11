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
    datetime: MyDateTime
}

impl Memo{
    pub fn new(id: u16)->Memo{
        Memo{id, ..Default::default()}
    }

    pub fn id(&self)->u16{
        self.id
    }

    pub fn parse_tags(&mut self){

        let mut offset = 0;
        while let Some(idx) = self.body[offset..].find("#"){
            offset = idx + 1;
            //todo ignore short hashtags
            self.body[offset..].find(|c: char|!c.is_alphabetic()).map(|tag_len|self.add_tag(self.body[offset..offset+tag_len].to_owned())); //note offset + 1 for ommiting the # itself
        }
        //todo: compare performance
        // for w in self.body.split(' '){
        //     if w.starts_with('#') && w.len() > 2{
            //todo: add trim and remove #
        //         self.tags.insert(w.to_string());
        //     }
        // }
    }

    pub fn add_tag(&mut self, tag: String)->bool{
        self.tags.insert(tag)
    }

    pub fn has_tag(&self, tag: &str)->bool{
        self.tags.contains(tag)
    }

    pub fn select<'a, 's>(&self, query: MemoSearchQuery<'_>) -> Option<&Self> {
        if match query {
            MemoSearchQuery::Header(s) => self.header.contains(s),
            MemoSearchQuery::Content(s) => self.body.contains(s),
            MemoSearchQuery::Tag(t) => self.has_tag(t),
            MemoSearchQuery::Topic(t) => &self.topic == t,          
        } {
            Some(self)
        } else {
            None
        }
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
        {
            res += &format!("from: {}\n", self.datetime.format_simple());
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

#[derive(Clone)]
pub enum MemoSearchQuery<'a> {
    Header(&'a str),
    Content(&'a str),
    Tag(&'a str),
    Topic(&'a str),
    // Before(DateTime)
}