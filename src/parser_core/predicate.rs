use crate::parser_core::*;

pub fn parse_if<'a, A, F>(p: Parser<'a, A>, pred: F) -> Parser<'a, A>
where
    A: Clone + 'a,
    F: Fn(A) -> bool + Sync + Send + 'a,
{
    {
        move |input: String, index: usize| match p.run(input.clone(), index) {
            Ok((i, v)) => {
                if pred(v.clone()) {
                    Ok((i, v))
                } else {
                    Err(format!(
                        "parse_if: Expected pred({}) to return true but returned false instead.",
                        input
                    ))
                }
            }
            err @ Err(_) => err,
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_if_always_true() {
        let p = parse_if(take(1), |_| true);
        assert_eq!(Ok((1, "a".to_string())), p.run("abc".to_string(), 0))
    }

    #[test]
    fn parse_if_always_false() {
        let p = parse_if(take(1), |_| false);
        assert!(p.run("abc".to_string(), 0).is_err())
    }

    #[test]
    fn parse_if_equal_to_something() {
        let p = parse_if(take(1), |c| c == "a".to_string());
        assert_eq!(Ok((1, "a".to_string())), p.run("abc".to_string(), 0))
    }
}
