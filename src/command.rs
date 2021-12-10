use std::str::FromStr;

pub enum Command {
    List,
    ListTopic(String),
    View(u16),
    Add,
    Edit,
}
impl Command{
    pub fn long_list() -> Vec<String> {
        [
            ("list"),
            ("list topic"),
            ("view"),
            ("add"),
            ("edit"),

            "exit"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    pub fn short_list() -> Vec<String> {
        [
            ("l"),
            ("lt"),
            ("v"),
            ("a"),
            ("e"),

            "x"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
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
