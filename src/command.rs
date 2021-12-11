use crate::memo::Memo;

// pub enum Command {
//     List(Option<u16>),
//     ListTopic(String),
//     Search(SearchBy),
//     Get(u16),
//     CreateNew(Memo),
//     Edit(u16),
// }

// impl Command{
//     pub fn long_list() -> Vec<String> {
//         [
//             "list",
//             "list topic",
//             "view",
//             "create",
//             "edit",
//             "save",
//             "clear",

//             "exit"
//         ]
//         .iter()
//         .map(|s| s.to_string())
//         .collect()
//     }
// }

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
