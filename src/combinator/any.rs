use crate::Parser;

pub fn any<'a, A>(parsers: Vec<Parser<'a, A>>) -> Parser<'a, A>
where
    A: Clone + 'a,
{
    {
        move |input: String, index: usize| {
            for p in parsers.clone() {
                let res = p.run(input.clone(), index);
                if res.is_ok() {
                    return res;
                }
            }

            return Err(input);
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn any_one_valid() {
        let p = any(vec![str("a"), str("b"), str("c")]);
        assert_eq!(Ok((1, "a".to_string())), p.run("abc".to_string(), 0))
    }

    #[test]
    fn any_none_valid() {
        let p = any(vec![str("a"), str("b"), str("c")]);
        assert_matches!(p.run("d".to_string(), 0), Err(_))
    }

    #[test]
    fn any_first_valid() {
        let p = any(vec![str("a"), str("ab"), str("c")]);
        assert_eq!(Ok((1, "a".to_string())), p.run("abc".to_string(), 0))
    }
}
