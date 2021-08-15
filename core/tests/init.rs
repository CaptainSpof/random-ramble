#[cfg(test)]
mod test {
    use maplit::hashmap;
    use pretty_assertions::assert_eq;
    use random_ramble::refactor::{Ramble, RambleKind, RambleMap, RandomRamble};
    use std::{collections::HashMap, hash::Hash, path::PathBuf};

    #[test]
    fn init_default() {
        let rr = RandomRamble::default();

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap::default(),
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_adjs() {
        let adjs = vec!["Happy", "Sad"];

        let rr = RandomRamble::new().with_adjs(adjs);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap(hashmap! { RambleKind::Adjective => vec![Ramble {
                    category: None,
                    values: vec!["Happy".into(), "Sad".into()]},
                ]}),
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
                rambles: RambleMap(hashmap! { RambleKind::Adjective => vec![Ramble {
                    category: None,
                    values: vec!["Pretty".into()]},
                ]}),
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
                rambles: RambleMap(hashmap! { RambleKind::Adjective => vec![Ramble {
                    category: None,
                    values: vec!["Kind".into(), "Ruthless".into()],
                }]}),
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
        let themes = vec!["King"];

        let rr = RandomRamble::new().with_themes(themes);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap(hashmap! { RambleKind::Theme => vec![Ramble {
                    category: None,
                    values: vec!["King".into()]},
                ]}),
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
                rambles: RambleMap(hashmap! { RambleKind::Theme => vec![Ramble {
                    category: None,
                    values: vec!["Toto".into()]},
                ]}),
                template: None,
                context: None,
            }
        );
    }

    fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
        map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
    }

    #[test]
    fn init_with_themes_from_file_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/tests/themes/foobar");

        let rr = match RandomRamble::new().with_themes_path(&path) {
            Ok(rr) => rr,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };

        assert!(keys_match(
            &rr.rambles.0,
            &hashmap! { RambleKind::Theme => vec![
                Ramble {
                    category: Some("foobar".into()),
                    values: vec!["foo".into(), "bar".into()]
                },
            ]}
        ));
    }

    #[test]
    fn init_with_themes_from_dir_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/tests/themes/");

        let rr = match RandomRamble::new().with_themes_path(&path) {
            Ok(rr) => rr,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };

        assert!(keys_match(
            &rr.rambles.0,
            &hashmap! { RambleKind::Theme => vec![
                Ramble {
                    category: Some("toto".into()),
                    values: vec!["Toto".into(), "Titi".into()]
                },
                Ramble {
                    category: Some("foobar".into()),
                    values: vec!["foo".into(), "bar".into()]
                },
            ]}
        ));
    }

    #[test]
    fn init_with_others() {
        let others = vec!["🦀"];

        let rr = RandomRamble::new().with_others("emoji", others);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap(hashmap! { RambleKind::Other("emoji") => vec![Ramble {
                    category: None,
                    values: vec!["🦀".into()],
                },
                ]}),
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
                rambles: RambleMap(hashmap! { RambleKind::Other("emoji") => vec![Ramble {
                    category: None,
                    values: vec!["🦀".into()],
                },
                ]}),
                template: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_template() {
        let rr = RandomRamble::new().with_template("A {{adj}} for {{theme}}");

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap::default(),
                template: Some("A {{adj}} for {{theme}}"),
                context: None,
            }
        );
    }
}
