mod common;

#[cfg(test)]
mod tests {
    use super::common::*;

    #[test]
    fn element_start_is() {
        assert_eq!(
            Ok((
                28,
                (
                    "div".to_string(),
                    vec![
                        ("x".to_string(), "1".to_string()),
                        ("y".to_string(), "2".to_string()),
                        ("z".to_string(), "3".to_string())
                    ]
                )
            )),
            element_start().run("< div  x=\"1\"  y =\"2\" z = \"3\"".to_string(), 0)
        )
    }

    #[test]
    fn leaf_element_raw_is() {
        assert_eq!(
            Ok((
                30,
                (
                    "div".to_string(),
                    vec![
                        ("x".to_string(), "1".to_string()),
                        ("y".to_string(), "2".to_string()),
                        ("z".to_string(), "3".to_string())
                    ]
                )
            )),
            leaf_element_raw().run("< div  x=\"1\"  y =\"2\" z = \"3\"/>".to_string(), 0)
        )
    }

    #[test]
    fn leaf_element_raw_malformed() {
        assert!(leaf_element_raw()
            .run("<div x=\"1/>".to_string(), 0)
            .is_err())
    }

    #[test]
    fn leaf_element_raw_malformed_2() {
        assert!(leaf_element_raw()
            .run("<div that's not an identifier/>".to_string(), 0)
            .is_err())
    }

    #[test]
    fn leaf_element_is() {
        assert_eq!(
            Ok((
                37,
                Element {
                    name: "div".to_string(),
                    attributes: vec![
                        ("x".to_string(), "1".to_string()),
                        ("y".to_string(), "2".to_string()),
                        ("z".to_string(), "3".to_string())
                    ],
                    children: vec![]
                }
            )),
            leaf_element().run("<div x=\"1\" y = \"2\" z = \"3\"         />".to_string(), 0)
        )
    }

    #[test]
    fn leaf_element_malformed_contain_children() {
        assert!(leaf_element()
            .run("<div x=\"1\" >...</div>".to_string(), 0)
            .is_err())
    }

    #[test]
    fn parent_element_is() {
        assert_eq!(
            Ok((
                36,
                Element {
                    name: "div".to_string(),
                    attributes: vec![
                        ("x".to_string(), "1".to_string()),
                        ("y".to_string(), "2".to_string()),
                        ("z".to_string(), "3".to_string())
                    ],
                    children: vec![]
                }
            )),
            parent_element().run("<div x=\"1\" y = \"2\" z = \"3\"         >".to_string(), 0)
        )
    }

    #[test]
    fn parent_element_is_contain_children() {
        assert_eq!(
            Ok((
                29,
                Element {
                    name: "a".to_string(),
                    attributes: vec![("href".to_string(), "https://google.com".to_string())],
                    children: vec![]
                }
            )),
            parent_element().run("<a href=\"https://google.com\">...</a>".to_string(), 0)
        )
    }
}
