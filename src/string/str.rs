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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn str_exact_input() {
        assert_eq!(
            Ok((5, "Hello".to_string())),
            str("Hello").run("Hello world!".to_string(), 0)
        )
    }

    #[test]
    fn str_non_exact_input() {
        assert!(str("Hewwo").run("Hello world!".to_string(), 0).is_err())
    }

    #[test]
    fn quoted_string_is() {
        assert_eq!(
            Ok((14, "Hello world!".to_string())),
            quoted_string().run("\"Hello world!\" he said".to_string(), 0)
        )
    }

    #[test]
    fn quoted_string_is_not() {
        assert!(quoted_string()
            .run("He said Hello world!".to_string(), 0)
            .is_err())
    }
}
