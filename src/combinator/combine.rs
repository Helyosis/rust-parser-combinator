#[allow(unused_imports)]
use crate::parser_core::TupleAppend;

#[macro_export]
macro_rules! combine {
    ( $($parser:expr),* ) => (
            Parser::from(
                move |input: String, index: usize| {
                let result = ();
                let mut next_index = index;
                $(
                    let input_: String = input.clone();

                    let res = $parser.run(input_, next_index);
                    if res.is_err() {return Err(res.unwrap_err());};
                    let (i, r) = res.unwrap();

                    next_index = i;
                    let result = result.append(r);

                )*

                Ok((next_index, result))
                }
            )
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn no_parser() {
        #[allow(unused_variables, unused_mut)]
        let p = combine!();
        assert_eq!(Ok((0, ())), p.run("AA".to_string(), 0))
    }

    #[test]
    fn one_parser_ok() {
        let p1 = str("Hey salut!");
        let p2 = combine!(str("Hey salut!"));

        let r = p1.run("Hey salut!".to_string(), 0).map(|(i, v)| (i, (v,)));
        assert_eq!(r, p2.run("Hey salut!".to_string(), 0))
    }

    #[test]
    fn one_parser_err() {
        let p1 = str("Hey salut!");
        let p2 = combine!(str("Hey salut!"));

        let r = p1
            .run("Hello world!".to_string(), 0)
            .map(|(i, v)| (i, (v,))); // To make the same type
        assert_eq!(r, p2.run("Hello world!".to_string(), 0))
    }

    #[test]
    fn two_parsers() {
        assert_eq!(
            Ok((2, ("1".to_string(), "2".to_string()))),
            combine!(str("1"), digit()).run("123".to_string(), 0)
        )
    }
}
