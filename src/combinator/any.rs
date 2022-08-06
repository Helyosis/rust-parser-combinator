use crate::Parser;

pub fn any<'a, A>(parsers: Vec<Parser<'a, A>>) -> Parser<'a, A>
where
    A: Clone + std::fmt::Debug + 'a,
{
    {
        move |input: String, index: usize| {
            let mut err: Result<(usize, A), String> = Err("any: No input parsers".to_string());

            for p in parsers.clone() {
                let res = p.run(input.clone(), index);
                if res.is_ok() {
                    return res;
                }
                err = res.clone();
            }

            return Err(format!(
                "any: No valid parser for parameters (\"{}\", {}). Last error was \"{}\"",
                input,
                index,
                err.unwrap_err()
            ));
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn any_one_valid() {
        let p = any(vec![str("a"), str("b"), str("c")]);
        assert_eq!(Ok((1, "a".to_string())), p.run("abc".to_string(), 0))
    }

    #[test]
    fn any_none_valid() {
        let p = any(vec![str("a"), str("b"), str("c")]);
        assert!(p.run("d".to_string(), 0).is_err())
    }

    #[test]
    fn any_first_valid() {
        let p = any(vec![str("a"), str("ab"), str("c")]);
        assert_eq!(Ok((1, "a".to_string())), p.run("abc".to_string(), 0))
    }
}
