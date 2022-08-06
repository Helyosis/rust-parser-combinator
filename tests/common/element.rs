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

pub fn parent_element<'a>() -> Parser<'a, Element> {
    left(element_start(), str(">")).map(|(n, attr)| Element {
        name: n,
        attributes: attr,
        children: vec![],
    })
}

fn parent_element_end<'a>(element_name: String) -> Parser<'a, ()> {
    combine!(str("</"), wrap_in_spaces(str(&*element_name)), str(">")).map(|_| ())
}

#[allow(dead_code)]
pub fn element<'a>() -> Parser<'a, Element> {
    let p = parent_element().and_then(|elem| {
        left(
            many(element()),
            pair(whitespaces(), parent_element_end(elem.name.clone())),
        )
        .map(move |children| Element {
            name: elem.name.clone(),
            attributes: elem.attributes.clone(),
            children,
        })
    });

    wrap_in_spaces(any(vec![leaf_element(), p]))
}
