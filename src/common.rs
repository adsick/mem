use chrono::Local;
pub use inquire::{Text, Select, Confirm};
pub use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
pub use hex::{encode, decode};
pub use rand::random;
pub use chrono::DateTime;
pub use serde::{Serialize, Deserialize};
pub use serde_json::{from_reader, from_str, to_string, to_string_pretty, to_writer_pretty};

#[derive(Serialize, Deserialize)]
pub struct MyDateTime(pub DateTime<Local>);

impl MyDateTime{
    pub fn format(&self, format_str: &str)->String{
        self.0.format(format_str).to_string()
    }
    pub fn format_simple(&self)->String{
        self.0.format("%F %R").to_string()
    }
}

impl Default for MyDateTime{
    fn default() -> Self {
        Self(Local::now())
    }
}