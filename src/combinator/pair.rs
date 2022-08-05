use crate::parser_core::*;

pub fn pair<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, (R1, R2)>
where
    R1: Clone + 'a,
    R2: Clone + 'a,
{
    {
        move |input: String, index| {
            parser1
                .run(input.clone(), index)
                .and_then(|(next_index, result1)| {
                    parser2
                        .run(input, next_index)
                        .map(|(final_index, result2)| (final_index, (result1, result2)))
                })
        }
    }
    .into()
}

pub fn left<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, R1>
where
    R1: Clone + 'a,
    R2: Clone + 'a,
{
    map(pair(parser1, parser2), |(left, _)| left)
}

pub fn right<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, R2>
where
    R1: Clone + 'a,
    R2: Clone + 'a,
{
    map(pair(parser1, parser2), |(_, right)| right)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn pair_ok() {
        assert_eq!(
            Ok((2, ("1".to_string(), "2".to_string()))),
            pair(str("1"), digit()).run("123".to_string(), 0)
        )
    }

    #[test]
    fn pair_err() {
        assert!(pair(str("1"), digit()).run("1a2".to_string(), 0).is_err())
    }

    #[test]
    fn pair_left() {
        assert_eq!(
            Ok((2, "1".to_string())),
            left(digit(), digit()).run("123".to_string(), 0)
        )
    }

    #[test]
    fn pair_right() {
        assert_eq!(
            Ok((2, "2".to_string())),
            right(digit(), digit()).run("123".to_string(), 0)
        )
    }
}
