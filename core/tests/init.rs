#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use maplit::hashmap;
    use random_ramble::refactor::{RambleKind, RandomRamble};

    #[test]
    fn init_default() {
        let rr = RandomRamble::default();

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: HashMap::new(),
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_adjs() {
        // let adjs = vec![
        //     Ramble::new("Happy").with_kind(RambleKind::Adjective),
        //     Ramble::new("Sad").with_kind(RambleKind::Adjective),
        // ];
        let adjs = vec![
            "Happy",
            "Sad"
        ];

        let rr = RandomRamble::new().with_adjs(adjs);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Adjective => vec!["Happy", "Sad"],
                },
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_adj_from_str() {
        let adj = "Pretty";

        let rr: RandomRamble = RandomRamble::default().with_adj(adj);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Adjective => vec!["Pretty"],
                },
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_adjs_from_string() {
        let adj1 = "Kind";
        let adj2 = "Ruthless";

        let rr = RandomRamble::new().with_adjs(vec![adj1, adj2]);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Adjective => vec!["Kind", "Ruthless"],
                },
                template: None,
                context: None,
            }
        );
    }

    // #[test]
    // fn init_with_adjs_from_path() {

    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("resources/tests/adjectives/");

    //     let rr = RandomRamble::new()
    //         .with_adjs_path(&path);

    //     assert_eq!(rr, RandomRamble {
    //         rambles: vec![
    //             Ramble_ {
    //                 kind: RambleKind_::Adjective,
    //                 value: "Ugly",
    //                 file: Some(File {
    //                     name: "test2",
    //                     path: format!("{}test2", path.clone().into_os_string().into_string().expect("🤷"))
    //                 })
    //             },
    //             Ramble_ {
    //                 kind: RambleKind_::Adjective,
    //                 value: "Pretty",
    //                 file: Some(File {
    //                     name: "test1",
    //                     path: format!("{}test1", path.clone().into_os_string().into_string().expect("🤷"))
    //                 })
    //             },
    //         ],
    //         template: None
    //     });
    // }

    #[test]
    fn init_with_themes() {
        // let themes = vec![Ramble::new("King").with_kind(RambleKind::Theme)];
        let themes = vec!["King"];

        let rr = RandomRamble::new().with_themes(themes);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Theme => vec!["King"],
                },
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_theme_from_string() {
        let theme = "Toto";

        let rr = RandomRamble::new().with_theme(theme);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Theme => vec!["Toto"],
                },
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_others() {
        // let others = vec![Ramble::new("🦀").with_kind(RambleKind::Other("emoji"))];
        let others = vec!["🦀"];

        let rr = RandomRamble::new().with_others("emoji", others);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Other("emoji") => vec!["🦀"],
                },
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_other_from_string() {
        let other = "🦀";

        let rr = RandomRamble::new().with_other("emoji", other);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: hashmap! {
                    RambleKind::Other("emoji") => vec!["🦀"],
                },
                template: None,
                context: None,
            }
        );
    }


    // #[test]
    // fn init_with_themes_from_path() {

    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("resources/tests/themes/");

    //     let rr = RandomRamble::new()
    //         .with_themes_path(&path);

    //     assert_eq!(rr, RandomRamble {
    //         rambles: vec![
    //             Ramble_ {
    //                 kind: RambleKind_::Theme,
    //                 value: "Titi",
    //                 file: Some(File {
    //                     name: "test2",
    //                     path: format!("{}test2", path.clone().into_os_string().into_string().expect("🤷"))
    //                 })
    //             },
    //             Ramble_ {
    //                 kind: RambleKind_::Theme,
    //                 value: "Toto",
    //                 file: Some(File {
    //                     name: "test1",
    //                     path: format!("{}test1", path.clone().into_os_string().into_string().expect("🤷"))
    //                 })
    //             },
    //         ],
    //         template: None
    //     });
    // }

    #[test]
    fn init_with_template() {
        let rr = RandomRamble::new().with_template("A {{adj}} for {{theme}}");

        assert_eq!(
            rr,
            RandomRamble {
                rambles: vec![],
                _rambles: HashMap::new(),
                template: Some("A {{adj}} for {{theme}}"),
                context: None,
            }
        );
    }
}
