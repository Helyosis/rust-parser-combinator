use crate::common::*;
use rust_parser_combinator::*;

pub fn element_start<'a>() -> Parser<'a, (String, Vec<(String, String)>)> {
    right(str("<"), pair(wrap_in_spaces(identifier()), attributes()))
}

pub fn leaf_element_raw<'a>() -> Parser<'a, (String, Vec<(String, String)>)> {
    left(element_start(), str(">"))
}
