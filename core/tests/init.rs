#[cfg(test)]
mod test {
    use maplit::hashmap;
    use pretty_assertions::assert_eq;
    use random_ramble::refactor::{Ramble, RambleKind, RambleMap, RandomRamble};
    use std::path::PathBuf;

    #[test]
    fn init_default() {
        let rr = RandomRamble::default();

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap::default(),
                context: None,
                templates: Vec::new(),
                tera: None,
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
                templates: Vec::new(),
                tera: None,
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
                templates: Vec::new(),
                tera: None,
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
                templates: Vec::new(),
                tera: None,
                context: None,
            }
        );
    }

    #[test]
    #[should_panic(expected = "The system cannot find the file specified.")]
    #[cfg(target_os = "windows")]
    fn fail_with_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("nope");

        RandomRamble::new().with_adjs_path(&path).unwrap();
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    #[cfg(not(target_os = "windows"))]
    fn fail_with_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("nope");

        RandomRamble::new().with_adjs_path(&path).unwrap();
    }

    #[test]
    fn init_with_adjectives_from_file_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/tests/adjectives/pt");

        let rr = match RandomRamble::new().with_adjs_path(&path) {
            Ok(rr) => rr,
            Err(e) => {
                panic!("{} {:#?}", e, e);
            }
        };

        assert!(&rr.rambles.0.eq(&hashmap! { RambleKind::Adjective => vec![
            Ramble {
                category: Some("pt".into()),
                values: vec!["Tímido".into()]
            },
        ]}));
    }

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
                templates: Vec::new(),
                tera: None,
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
                templates: Vec::new(),
                tera: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_themes_from_file_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/tests/themes/country");

        let rr = match RandomRamble::new().with_themes_path(&path) {
            Ok(rr) => rr,
            Err(e) => {
                panic!("{} {:#?}", e, e);
            }
        };

        assert!(&rr.rambles.0.eq(&hashmap! { RambleKind::Theme => vec![
            Ramble {
                category: Some("country".into()),
                values: vec!["Portugal".into()]
            },
        ]}));
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

        // TODO: actually test stuff
        assert_eq!(rr.rambles.0.len(), 1);
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
                templates: Vec::new(),
                tera: None,
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
                templates: Vec::new(),
                tera: None,
                context: None,
            }
        );
    }

    #[test]
    fn init_with_others_from_file_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/tests/others/greetings/fr");

        let rr = match RandomRamble::new().with_others_path("greetings", &path) {
            Ok(rr) => rr,
            Err(e) => {
                panic!("{} {:#?}", e, e);
            }
        };

        assert!(&rr
            .rambles
            .0
            .eq(&hashmap! { RambleKind::Other("greetings") => vec![
                Ramble {
                    category: Some("fr".into()),
                    values: vec!["Bonjour".into()]
                },
            ]}));
    }

    #[test]
    fn init_with_template() {
        let rr = RandomRamble::new().with_templates(vec!["A {{adj}} for {{theme}}"]);

        assert_eq!(
            rr,
            RandomRamble {
                rambles: RambleMap::default(),
                templates: vec!["A {{adj}} for {{theme}}"],
                tera: None,
                context: None,
            }
        );
    }
}
