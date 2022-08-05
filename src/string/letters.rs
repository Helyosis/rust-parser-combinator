use crate::*;

pub fn letter<'a>() -> Parser<'a, String> {
    {
        move |input: String, index: usize| match input.chars().nth(index) {
            Some(c) if c.is_alphabetic() => Ok((index + 1, c.to_string())),
            Some(c) => Err(format!("letter: {} is not alphabetic", c)),
            None => Err(format!(
                "letter: Invalid index of {} for input string of size {}",
                index,
                input.len()
            )),
        }
    }
    .into()
}

pub fn lowercase_letter<'a>() -> Parser<'a, String> {
    {
        move |input: String, index: usize| match input.chars().nth(index) {
            Some(c) if c.is_alphabetic() && c.is_lowercase() => Ok((index + 1, c.to_string())),
            Some(c) => Err(format!("letter: {} is not alphabetic and lowercase", c)),
            None => Err(format!(
                "letter: Invalid index of {} for input string of size {}",
                index,
                input.len()
            )),
        }
    }
    .into()
}

pub fn uppercase_letter<'a>() -> Parser<'a, String> {
    {
        move |input: String, index: usize| match input.chars().nth(index) {
            Some(c) if c.is_alphabetic() && c.is_uppercase() => Ok((index + 1, c.to_string())),
            Some(c) => Err(format!("letter: {} is not alphabetic and uppercase", c)),
            None => Err(format!(
                "letter: Invalid index of {} for input string of size {}",
                index,
                input.len()
            )),
        }
    }
    .into()
}

pub fn alphanum<'a>() -> Parser<'a, String> {
    {
        move |input: String, index: usize| match input.chars().nth(index) {
            Some(c) if c.is_alphabetic() && c.is_alphanumeric() => Ok((index + 1, c.to_string())),
            Some(c) => Err(format!("letter: {} is not alphanumeric", c)),
            None => Err(format!(
                "letter: Invalid index of {} for input string of size {}",
                index,
                input.len()
            )),
        }
    }
    .into()
}

pub fn letters<'a>() -> Parser<'a, String> {
    many(letter()).map(|v| v.concat())
}

pub fn lowercase_letters<'a>() -> Parser<'a, String> {
    many(lowercase_letter()).map(|v| v.concat())
}

pub fn uppercase_letters<'a>() -> Parser<'a, String> {
    many(uppercase_letter()).map(|v| v.concat())
}

pub fn alphanums<'a>() -> Parser<'a, String> {
    many(alphanum()).map(|v| v.concat())
}
