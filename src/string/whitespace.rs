use crate::Parser;
use crate::{many, many1};

pub fn whitespace() -> Parser<'static, String> {
    {
        move |input: String, index: usize|
        match input.chars().nth(index) {
            Some(c) if c.is_whitespace() => Ok((index + 1, c.to_string())),
            Some(c) => Err(format!("whitespace: expected a whitespace at position {} but got \"{}\"", index, c)),
            None => Err(format!("whitespace: expected a whitespace at position {} but input string is too short (len = {})", index, input.len()))
        }
    }.into()
}

pub fn whitespaces() -> Parser<'static, String> {
    many(whitespace()).map(|v| v.join(""))
}

pub fn whitespaces1() -> Parser<'static, String> {
    many1(whitespace()).map(|v| v.join(""))
}
