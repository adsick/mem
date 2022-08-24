use std::str::FromStr;

use crate::common::*;
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
    pub fn new(text: String) -> Self {
        let paragraphs = vec![text]; // todo

        Self { paragraphs }
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
