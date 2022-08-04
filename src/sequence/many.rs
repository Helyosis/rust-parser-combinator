use crate::parser_core::*;

pub fn many<'a, OutputType>(parser: Parser<'a, OutputType>) -> Parser<Vec<OutputType>>
where
    OutputType: Clone + 'a,
{
    {
        move |input: String, index| {
            let mut results: Vec<OutputType> = vec![];
            let mut i = index;
            loop {
                match parser.run(input.clone(), i) {
                    Ok((new_index, r)) => {
                        i = new_index;
                        results.push(r)
                    }
                    _ => return Ok((i, results)),
                }
            }
        }
    }
    .into()
}

pub fn many1<'a, OutputType>(parser: Parser<'a, OutputType>) -> Parser<Vec<OutputType>>
where
    OutputType: Clone + 'a,
{
    {
        move |input: String, index| match many(parser.clone()).run(input.clone(), index)? {
            (i, v) if v.len() > 0 => Ok((i, v)),
            _ => Err(input.clone().to_string()),
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn many_none() {
        let p = many(digit());
        assert_eq!(Ok((0, vec![])), p.run("".to_string(), 0))
    }

    #[test]
    fn many_one() {
        let p = many(digit());
        assert_eq!(Ok((1, vec!["1".to_string()])), p.run("1a".to_string(), 0))
    }

    #[test]
    fn many_several() {
        let p = many(digit());
        assert_eq!(
            Ok((3, vec!["1".to_string(), "2".to_string(), "3".to_string()])),
            p.run("123".to_string(), 0)
        )
    }

    #[test]
    fn many1_none() {
        let p = many1(digit());
        assert!(p.run("".to_string(), 0).is_err())
    }

    #[test]
    fn many1_one() {
        let p = many1(digit());
        assert_eq!(Ok((1, vec!["1".to_string()])), p.run("1a".to_string(), 0))
    }

    #[test]
    fn many1_several() {
        let p = many1(digit());
        assert_eq!(
            Ok((3, vec!["1".to_string(), "2".to_string(), "3".to_string()])),
            p.run("123".to_string(), 0)
        )
    }
}
