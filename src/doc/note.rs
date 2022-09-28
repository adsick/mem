use std::str::FromStr;

use crate::parsing::paragraph;
use crate::{common::*, split_paragraphs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    paragraphs: Vec<String>,
    // web links
    // links: Vec<Link>,
    // mem refs
    // references: Vec<Reference>,
}

impl Note {
    pub fn new(text: String) -> Result<Self> {
        let paragraphs = split_paragraphs(&text)?
            .into_iter()
            .map(str::to_string)
            .collect();

        Ok(Self { paragraphs })
    }

    pub fn add_paragraph(&mut self, paragraph: String) -> &mut Self {
        self.paragraphs.push(paragraph);
        self
    }

    pub fn paragraphs(&self) -> &[String] {
        &self.paragraphs
    }

    pub fn paragraph(&self, n: usize) -> Option<&String> {
        self.paragraphs.get(n)
    }
}

impl FromStr for Note {
    type Err = ErrReport;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // not the best solution I guess
        let paragraphs = split_paragraphs(s)?.iter().map(|s| s.to_string()).collect();
        Ok(Note { paragraphs })
    }
}

// impl From<String> for Note {
//     fn from(str: String) -> Self {
//         // this is cringe

//     }
// }
