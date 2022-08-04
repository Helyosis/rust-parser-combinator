use crate::{between, repeat_until, take, Parser};

pub fn str<'a>(expected: &'a str) -> Parser<String> {
    let len = expected.len();

    {
        move |input: String, index| match input.get(index..index + len) {
            Some(next) if next == expected => Ok((index + expected.len(), next.to_string())),
            Some(got) => Err(format!(
                "str: Expected {} at index {} but got {}",
                expected, index, got
            )),
            _ => Err(format!(
                "str: Expected input size of at least {} but got {}",
                index + len,
                input.len()
            )),
        }
    }
    .into()
}

pub fn quoted_string<'a>() -> Parser<'a, String> {
    between(
        str("\""),
        str("\""),
        repeat_until(take(1), |s| s == "\"").map(|v| v.join("")),
    )
}
