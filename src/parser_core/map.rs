use crate::parser_core::*;

pub fn map<'a, F, A, B>(parser: Parser<'a, A>, transformation_fn: F) -> Parser<B>
where
    A: Clone + 'a,
    B: Clone + 'a,
    F: Fn(A) -> B + Sync + Send + 'a,
{
    {
        move |input, index| {
            parser
                .run(input, index)
                .map(|(next_index, output)| (next_index, transformation_fn(output)))
        }
    }
    .into()
}

pub fn map_err<'a, F, A>(parser: Parser<'a, A>, transformation_fn: F) -> Parser<A>
where
    A: Clone + 'a,
    F: Fn(String) -> String + Sync + Send + 'a,
{
    {
        move |input, index| {
            parser
                .run(input, index)
                .map_err(|err| (transformation_fn(err)))
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn map_identity() {
        let p1 = digits().map(|x| x);
        let p2 = digits();

        let input_str = "1234Hey !".to_string();
        assert_eq!(p1.run(input_str.clone(), 0), p2.run(input_str, 0))
    }

    #[test]
    fn map_uppercase() {
        let p = str("Hello world!").map(|s| s.to_uppercase());

        let input_str = "Hello world!".to_string();
        assert_eq!(p.run(input_str, 0), Ok((12, "HELLO WORLD!".to_string())))
    }

    #[test]
    fn map_no_impact_on_err() {
        let p1 = str("Hello world!").map(|s| s.to_uppercase());
        let p2 = str("Hello world!");

        let input_str = "Goodbye world!".to_string();
        assert_eq!(p1.run(input_str.clone(), 0), p2.run(input_str, 0))
    }

    #[test]
    fn map_err_identity() {
        let p1 = str("Hello world!").map_err(|x| x);
        let p2 = str("Hello world!");

        let input_str = "Goodbye world!".to_string();
        assert!(p1.clone().run(input_str.clone(), 0).is_err()); // Sanity check
        assert_eq!(p1.run(input_str.clone(), 0), p2.run(input_str, 0))
    }

    #[test]
    fn map_err_replace() {
        let p = str("Hello world!").map_err(|_| "test: placeholder".to_string());

        assert_eq!(
            Err("test: placeholder".to_string()),
            p.run("Goodbye world!".to_string(), 0)
        )
    }

    #[test]
    fn map_err_no_impact_on_ok() {
        let p1 = str("Hello world!").map_err(|_| "test: placeholder".to_string());
        let p2 = str("Hello world!");

        let input_str = "Hello world!".to_string();

        assert!(p1.clone().run(input_str.clone(), 0).is_ok()); // Sanity check
        assert_eq!(p1.run(input_str.clone(), 0), p2.run(input_str, 0))
    }
}
