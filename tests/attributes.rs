mod common;

#[cfg(test)]
mod tests {
    use super::common::*;

    #[test]
    fn identifier_one_character() {
        assert_eq!(
            Ok((1, "x".to_string())),
            identifier().run("x=1".to_string(), 0)
        )
    }

    #[test]
    fn identifier_is() {
        assert_eq!(
            Ok((13, "myAt-trib_ute".to_string())),
            identifier().run("myAt-trib_ute=1".to_string(), 0)
        )
    }

    #[test]
    fn identifier_nonalpha_in_first() {
        assert!(identifier().run("~Hey=1".to_string(), 0).is_err())
    }

    #[test]
    fn attribute_pair_is() {
        assert_eq!(
            Ok((6, ("x".to_string(), ":)".to_string()))),
            attribute_pair().run("x=\":)\"".to_string(), 0)
        )
    }

    #[test]
    fn attribute_pair_with_whitespaces() {
        assert_eq!(
            Ok((9, ("x".to_string(), ":)".to_string()))),
            attribute_pair().run("x =  \":)\"  ".to_string(), 0)
        )
    }

    #[test]
    fn attribute_pair_no_quotes() {
        assert!(attribute_pair().run("x = :)".to_string(), 0).is_err())
    }

    #[test]
    fn attributes_is() {
        assert_eq!(
            Ok((
                25,
                vec![
                    ("x".to_string(), "1".to_string()),
                    ("y".to_string(), "2".to_string()),
                    ("z".to_string(), "3".to_string())
                ]
            )),
            attributes().run("  x = \"1\"  y = \"2\"  z=\"3\"".to_string(), 0)
        )
    }

    #[test]
    fn attributes_is_not() {
        // It's expected to return Ok()
        // The error will happen later, but because we are using many() in attributes(), it does not throw an error.
        assert_eq!(Ok((0, vec![])), attributes().run("x = 1".to_string(), 0))
    }
}
