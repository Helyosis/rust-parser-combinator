use crate::parser_core::*;

pub fn repeat_while<'a, A, F>(p: Parser<'a, A>, predicate: F) -> Parser<'a, Vec<A>>
where
    A: Clone + 'a,
    F: Fn(&A) -> bool + Send + Sync + 'a,
{
    {
        move |input, index: usize| {
            let input: String = input;
            let mut res: Vec<A> = vec![];
            let mut i = index;
            loop {
                match p.run(input.clone(), i) {
                    Ok((new_i, content)) if predicate(&content) => {
                        i = new_i;
                        res.push(content)
                    }
                    _ => return Ok((i, res)),
                }
            }
        }
    }
    .into()
}

pub fn repeat_until<'a, A, F>(p: Parser<'a, A>, predicate: F) -> Parser<'a, Vec<A>>
where
    A: Clone + 'a,
    F: Fn(&A) -> bool + Send + Sync + 'a,
{
    repeat_while(p, move |x: &A| !predicate(x))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn repeat_while_always_true() {
        let p = repeat_while(digit(), |_| true);

        assert_eq!(
            Ok((3, vec!["1".to_string(), "2".to_string(), "3".to_string()])),
            p.run("123".to_string(), 0)
        );
    }

    #[test]
    fn repeat_while_always_false() {
        let p = repeat_while(digit(), |_| false);

        assert_eq!(Ok((0, vec![])), p.run("123".to_string(), 0));
    }

    #[test]
    fn repeat_while_even_digit() {
        let p = repeat_while(digit().map(|c| c.parse::<i32>().unwrap()), |d| d % 2 == 0);

        assert_eq!(Ok((2, vec![0, 2])), p.run("02123".to_string(), 0))
    }

    #[test]
    fn repeat_until_always_true() {
        let p = repeat_until(digit(), |_| true);

        assert_eq!(Ok((0, vec![])), p.run("123".to_string(), 0));
    }

    #[test]
    fn repeat_until_always_false() {
        let p = repeat_until(digit(), |_| false);

        assert_eq!(
            Ok((3, vec!["1".to_string(), "2".to_string(), "3".to_string()])),
            p.run("123".to_string(), 0)
        );
    }

    #[test]
    fn repeat_until_even_digit() {
        let p = repeat_until(digit().map(|c| c.parse::<i32>().unwrap()), |d| d % 2 == 0);

        assert_eq!(Ok((1, vec![1])), p.run("123".to_string(), 0))
    }
}
