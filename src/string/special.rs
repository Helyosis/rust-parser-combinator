use crate::parser_core::*;

pub fn nothing() -> Parser<'static, String> {
    { |_: String, index: usize| Ok((index, "".to_string())) }.into()
}

pub fn everything() -> Parser<'static, String> {
    {
        |input: String, index: usize| {
            Ok((
                input.len(),
                input.get(index..).unwrap_or_default().to_string(),
            ))
        }
    }
    .into()
}

pub fn start_of_input() -> Parser<'static, String> {
    {
        |_: String, index: usize| match index {
            0 => Ok((0, "".to_string())),
            _ => Err(format!(
                "start_of_input: Expected index to be at index 0, got index {}",
                index
            )),
        }
    }
    .into()
}

pub fn end_of_input() -> Parser<'static, String> {
    {
        |input: String, index: usize| {
            if index == input.len() {
                Ok((index, "".to_string()))
            } else {
                Err(format!(
                    "end_of_input: Expected index to be at end of input (= {}), got index {}",
                    input.len(),
                    index
                ))
            }
        }
    }
    .into()
}

pub fn take(n: usize) -> Parser<'static, String> {
    {
        move |input: String, index: usize| match input.get(index..index + n) {
            Some(val) => Ok((index + n, val.to_string())),
            None => Err(format!(
                "take: Wanted to take {} char(s) at index {} but string was too short (len = {}) or was not on unicode boundaries.",
                n,
                index,
                input.len()
            )),
        }
    }
    .into()
}
