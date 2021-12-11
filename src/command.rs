use std::str::FromStr;

use crate::memo::Memo;

pub enum SearchBy{
    Any(String), 
    Text(String),
    Hashtag(String),
    Topic(String)
}

pub enum Command {
    List(Option<u16>),
    ListTopic(String),
    Search(SearchBy),
    Get(u16),
    CreateNew(Memo),
    Edit(u16),
}

impl Command{
    pub fn long_list() -> Vec<String> {
        [
            "list",
            "list topic",
            "view",
            "create",
            "edit",
            "save",
            "clear",

            "exit"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    // pub fn short_list() -> Vec<String> {
    //     [
    //         "l",
    //         "lt",
    //         "v",
    //         "c",
    //         "e",

    //         "x"
    //     ]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect()
    // }
}

// impl FromStr for Command{
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s{
//             "l" => Ok(Self::List),
//             "lt" => Ok(Self::ListTopic())
//             _ => Err("Command::from_str error.")
//         }
//     }
// }
