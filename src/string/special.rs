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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn nothing_start_of_input() {
        assert_eq!(
            Ok((0, "".to_string())),
            nothing().run("Hello world!".to_string(), 0)
        )
    }

    #[test]
    fn everything_end_of_input() {
        assert_eq!(
            Ok((12, "".to_string())),
            everything().run("Hello world!".to_string(), 12)
        )
    }

    #[test]
    fn start_of_input_is() {
        assert_eq!(
            Ok((0, "".to_string())),
            start_of_input().run("Hello world!".to_string(), 0)
        )
    }

    #[test]
    fn end_of_input_is() {
        assert_eq!(
            Ok((12, "".to_string())),
            end_of_input().run("Hello world!".to_string(), 12)
        )
    }

    #[test]
    fn start_of_input_is_not() {
        assert!(start_of_input().run("Hello world!".to_string(), 1).is_err())
    }

    #[test]
    fn end_of_input_is_not() {
        assert!(end_of_input().run("Hello world!".to_string(), 1).is_err())
    }

    #[test]
    fn take_one() {
        assert_eq!(
            Ok((1, "H".to_string())),
            take(1).run("Hello world!".to_string(), 0)
        )
    }

    #[test]
    fn take_too_many() {
        assert!(take(100).run("Hello world!".to_string(), 0).is_err())
    }
}
