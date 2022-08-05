use rust_parser_combinator::*;

pub fn identifier<'a>() -> Parser<'a, String> {
    pair(letter(), many(any(vec![alphanum(), str("-"), str("_")])))
        .map(|(start, end)| start + &end.concat())
}

pub fn attribute_pair<'a>() -> Parser<'a, (String, String)> {
    around(identifier(), quoted_string(), wrap_in_spaces(str("=")))
}

pub fn attributes<'a>() -> Parser<'a, Vec<(String, String)>> {
    many(wrap_in_spaces(attribute_pair()))
}
