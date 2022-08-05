use std::vec;

use crate::common::*;
use rust_parser_combinator::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Element>,
}

pub fn element_start<'a>() -> Parser<'a, (String, Vec<(String, String)>)> {
    right(str("<"), pair(wrap_in_spaces(identifier()), attributes()))
}

pub fn leaf_element_raw<'a>() -> Parser<'a, (String, Vec<(String, String)>)> {
    left(element_start(), str("/>"))
}

pub fn leaf_element<'a>() -> Parser<'a, Element> {
    leaf_element_raw().map(|(n, attr)| Element {
        name: n,
        attributes: attr,
        children: vec![],
    })
}
