use crate::{combine, parser_core::*};
use std::fmt::Debug;

pub fn triplet<'a, R1, R2, R3>(
    parser1: Parser<'a, R1>,
    parser2: Parser<'a, R2>,
    parser3: Parser<'a, R3>,
) -> Parser<'a, (R1, R2, R3)>
where
    R1: Clone + 'a + Debug,
    R2: Clone + 'a + Debug,
    R3: Clone + 'a + Debug,
{
    combine!(parser1, parser2, parser3)
}

pub fn between<'a, R1, R2, R3>(
    left: Parser<'a, R1>,
    right: Parser<'a, R2>,
    parser: Parser<'a, R3>,
) -> Parser<'a, R3>
where
    R1: Clone + 'a + Debug,
    R2: Clone + 'a + Debug,
    R3: Clone + 'a + Debug,
{
    map(triplet(left, parser, right), |(_, value, _)| value)
}

pub fn around<'a, R1, R2, R3>(
    left: Parser<'a, R1>,
    right: Parser<'a, R3>,
    parser: Parser<'a, R2>,
) -> Parser<'a, (R1, R3)>
where
    R1: Clone + 'a + Debug,
    R2: Clone + 'a + Debug,
    R3: Clone + 'a + Debug,
{
    map(triplet(left, parser, right), |(left, _, right)| {
        (left, right)
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::*;

    #[test]
    fn triplet_ok() {
        assert_eq!(
            Ok((4, ("1".to_string(), "2".to_string(), "34".to_string()))),
            triplet(str("1"), digit(), digits()).run("1234".to_string(), 0)
        )
    }

    #[test]
    fn triplet_err() {
        assert_matches!(
            triplet(str("1"), digit(), digits()).run("1a".to_string(), 0),
            Err(_)
        )
    }

    #[test]
    fn triplet_between() {
        assert_eq!(
            Ok((4, "2".to_string())),
            between(digit(), digits(), digit()).run("1234".to_string(), 0)
        )
    }

    #[test]
    fn triplet_around() {
        assert_eq!(
            Ok((4, ("1".to_string(), "34".to_string()))),
            around(digit(), digits(), digit()).run("1234".to_string(), 0)
        )
    }
}
