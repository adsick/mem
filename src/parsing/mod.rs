use crate::common::*;

use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::satisfy;

use nom::combinator::{recognize, rest, verify, opt};

use nom::multi::separated_list1;
use nom::multi::{many0, many1, many_m_n};

use nom::sequence::{pair, terminated};

use nom::error::Error;
use nom::error::ErrorKind;
use nom::IResult;

pub fn split_paragraphs(text: &str) -> Result<Vec<&str>> {
    match many1(paragraph)(text) {
        Ok((_, v)) => Ok(v),
        Err(e) => Err(ErrReport::msg(format!("err splitting paragraphs: {e:?}\n"))),
    }
}

#[deprecated]
pub fn line0(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, line_ending)(input)
}

pub fn line(input: &str) -> IResult<&str, &str> {
    let non_empty = |s: &str| s.len() > 0;
    recognize(terminated(verify(not_line_ending, non_empty), opt(line_ending)))(input)
}

pub fn paragraph(input: &str) -> IResult<&str, &str> {
    terminated(recognize(many1(line)), many0/*_count*/(line_ending))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_paragraphs_0() {
        let txt = "";
        split_paragraphs(txt).unwrap_err();
    }

    #[test]
    fn split_paragraphs_1() {
        let txt = "Hello, world!\nnext line\n";
        assert_eq!(vec![txt], split_paragraphs(txt).unwrap());
    }

    #[test]
    fn split_paragraphs_2() {
        let txt = "First paragraph\nsecond line\n\nSecond paragraph";
        let res = vec!["First paragraph\nsecond line\n", "Second paragraph"];
        assert_eq!(res, split_paragraphs(txt).unwrap());
    }

    #[test]
    fn split_paragraphs_3() {
        let txt = "1\nsecond line\n\n2\nsecond line\nthird line\n\n3\n";
        let res = vec!["1\nsecond line\n", "2\nsecond line\nthird line\n", "3\n"];
        assert_eq!(res, split_paragraphs(txt).unwrap());
    }
}
