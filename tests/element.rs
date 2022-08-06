mod common;

#[cfg(test)]
mod tests {
    use std::vec;

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
    fn leaf_element_is_no_attributes() {
        assert_eq!(
            Ok((
                6,
                Element {
                    name: "a".to_string(),
                    attributes: vec![],
                    children: vec![]
                }
            )),
            leaf_element().run("< a />".to_string(), 0)
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

    #[test]
    fn element_leaf_element() {
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
            element().run("<div x=\"1\" y = \"2\" z = \"3\"         />".to_string(), 0)
        )
    }

    #[test]
    fn element_one_child() {
        assert_eq!(
            Ok((
                22,
                Element {
                    name: "a".to_string(),
                    attributes: vec![],
                    children: vec![Element {
                        name: "b".to_string(),
                        attributes: vec![],
                        children: vec![]
                    }]
                }
            )),
            element().run("< a >  < b /> </ a  > ".to_string(), 0)
        )
    }

    #[test]
    fn element_multiple_children() {
        assert_eq!(
            Ok((
                42,
                Element {
                    name: "a".to_string(),
                    attributes: vec![("x".to_string(), "1".to_string())],
                    children: vec![
                        Element {
                            name: "b".to_string(),
                            attributes: vec![("y".to_string(), "2".to_string())],
                            children: vec![]
                        },
                        Element {
                            name: "c".to_string(),
                            attributes: vec![("z".to_string(), "3".to_string())],
                            children: vec![]
                        }
                    ]
                }
            )),
            element().run(
                "<a x=\"1\"><b y  =  \"2\" /> <c z=\"3\"  /></a >".to_string(),
                0
            )
        )
    }

    #[test]
    fn element_depth() {
        assert_eq!(
            Ok((
                30,
                Element {
                    name: "a".to_string(),
                    attributes: vec![],
                    children: vec![Element {
                        name: "b".to_string(),
                        attributes: vec![],
                        children: vec![Element {
                            name: "c".to_string(),
                            attributes: vec![],
                            children: vec![]
                        }]
                    }]
                }
            )),
            element().run("<a> <b> <c>  </ c> </ b> </a >".to_string(), 0)
        )
    }

    #[test]
    fn element_is_big() {
        // This test was copied from the source: https://bodil.lol/parser-combinators/#we-re-finally-there
        let doc = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;
        let parsed_doc = Element {
            name: "top".to_string(),
            attributes: vec![("label".to_string(), "Top".to_string())],
            children: vec![
                Element {
                    name: "semi-bottom".to_string(),
                    attributes: vec![("label".to_string(), "Bottom".to_string())],
                    children: vec![],
                },
                Element {
                    name: "middle".to_string(),
                    attributes: vec![],
                    children: vec![Element {
                        name: "bottom".to_string(),
                        attributes: vec![("label".to_string(), "Another bottom".to_string())],
                        children: vec![],
                    }],
                },
            ],
        };
        assert_eq!(Ok((175, parsed_doc)), element().run(doc.to_string(), 0));
    }

    #[test]
    fn element_err_mismatched_closing_tag() {
        let doc = r#"
        <top>
            <bottom/>
        </middle>"#;
        assert!(element().run(doc.to_string(), 0).is_err())
    }
}
