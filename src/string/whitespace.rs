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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn whitespace_is() {
        assert_eq!(
            Ok((1, "\n".to_string())),
            whitespace().run("\nHello world!".to_string(), 0)
        )
    }

    #[test]
    fn whitespace_is_not() {
        assert!(whitespace().run("Hello world!".to_string(), 0).is_err())
    }

    #[test]
    fn whitespaces_several() {
        assert_eq!(
            Ok((3, " \t\n".to_string())),
            whitespaces().run(" \t\nHey !".to_string(), 0)
        )
    }

    #[test]
    fn whitespaces1_several() {
        assert_eq!(
            Ok((3, " \t\n".to_string())),
            whitespaces1().run(" \t\nHey !".to_string(), 0)
        )
    }

    #[test]
    fn whitespaces1_none() {
        assert!(whitespaces1().run("Hey !".to_string(), 0).is_err())
    }
}
