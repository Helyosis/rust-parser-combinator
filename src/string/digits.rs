use crate::parser_core::*;
use crate::sequence::many;

pub fn digit_with_base<'a>(base: u32) -> Parser<'a, String> {
    {
        move |input: String, index| match input.chars().nth(index) {
            Some(c) if c.is_digit(base) => Ok((index + 1, c.to_string())),
            Some(c) => Err(format!(
                "digit_with_base: Expected {} to be a digit in base {} at index {} but it wasn't",
                c, base, index
            )),
            None => Err(format!(
                "digit_with_base: Expected a character to be present at index {} but got nothing",
                index
            )),
        }
    }
    .into()
}

pub fn digits_with_base<'a>(base: u32) -> Parser<'a, String> {
    map(many(digit_with_base(base)), |v| v.concat())
}

pub fn digit() -> Parser<'static, String> {
    digit_with_base(10)
}
pub fn digits() -> Parser<'static, String> {
    digits_with_base(10)
}
pub fn hexdigit() -> Parser<'static, String> {
    digit_with_base(16)
}
pub fn hexdigits() -> Parser<'static, String> {
    digits_with_base(16)
}

#[cfg(test)]
mod test {
    #[test]
    fn digit() {
        assert_eq!(
            Ok((1, "1".to_string())),
            crate::digit().run("123".to_string(), 0)
        )
    }

    #[test]
    fn digits() {
        assert_eq!(
            Ok((3, "123".to_string())),
            crate::digits().run("123".to_string(), 0)
        )
    }

    #[test]
    fn hexdigit() {
        assert_eq!(
            Ok((1, "f".to_string())),
            crate::hexdigit().run("f00dbabe".to_string(), 0)
        )
    }

    #[test]
    fn hexdigits() {
        assert_eq!(
            Ok((8, "f00dbabe".to_string())),
            crate::hexdigits().run("f00dbabe".to_string(), 0)
        )
    }
}
