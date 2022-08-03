use crate::parser_core::*;
use crate::sequence::many;

pub fn digit_with_base<'a>(base: u32)
    -> Parser<'a, String>
{
    {
        move |input: String, index| 
            match input.chars().nth(index) {
                Some(c) if c.is_digit(base) => Ok((index + 1, c.to_string())),
                Some(c) => Err(format!("digit_with_base: Expected {} to be a digit in base {} at index {} but it wasn't", c, base, index)),
                None => Err(format!("digit_with_base: Expected a character to be present at index {} but got nothing", index))
            }
    }.into()
}

pub fn digits_with_base<'a>(base: u32)
    -> Parser<'a, String>
{
    map(many(digit_with_base(base)), |v| v.concat())
}

pub fn digit() -> Parser<'static, String> { digit_with_base(10) }
pub fn digits() -> Parser<'static, String> { digits_with_base(10) }
pub fn hexdigit() -> Parser<'static, String> { digit_with_base(16) }
pub fn hexdigits() -> Parser<'static, String> { digits_with_base(16) }