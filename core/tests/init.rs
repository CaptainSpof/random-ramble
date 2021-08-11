use std::{collections::HashMap, path::PathBuf};

use random_ramble::refactor::{Ramble, RambleKind, RandomRamble};

#[test]
fn init_default() {
    let rr = RandomRamble::default();

    assert_eq!(
        rr,
        RandomRamble {
            rambles: vec![],
            _rambles: HashMap::new(),
            template: None
        }
    );
}

#[test]
fn init_with_adjs() {
    let adjs = vec![
        Ramble::new("Happy").with_kind(RambleKind::Adjective),
        Ramble::new("Sad").with_kind(RambleKind::Adjective),
    ];

    let rr = RandomRamble::new().with_adjs(adjs);

    assert_eq!(
        rr,
        RandomRamble {
            rambles: vec![
                Ramble {
                    kind: RambleKind::Adjective,
                    value: "Happy",
                    file: None
                },
                Ramble {
                    kind: RambleKind::Adjective,
                    value: "Sad",
                    file: None
                },
            ],
            _rambles: HashMap::new(),
            template: None
        }
    );
}

#[test]
fn init_with_adj_from_string() {
    let adj = "Pretty";

    let rr: RandomRamble = RandomRamble::default().with_adj(adj.into());

    assert_eq!(
        rr,
        RandomRamble {
            rambles: vec![Ramble {
                kind: RambleKind::Adjective,
                value: "Pretty",
                file: None
            }],
            _rambles: HashMap::new(),
            template: None
        }
    );
}

#[test]
fn init_with_adjs_from_string() {
    let adj1 = "Kind";
    let adj2 = "Ruthless";

    let rr = RandomRamble::new().with_adjs(vec![adj1.into(), adj2.into()]);

    assert_eq!(
        rr,
        RandomRamble {
            rambles: vec![
                Ramble {
                    kind: RambleKind::Adjective,
                    value: "Kind",
                    file: None
                },
                Ramble {
                    kind: RambleKind::Adjective,
                    value: "Ruthless",
                    file: None
                },
            ],
            _rambles: HashMap::new(),
            template: None
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
    let themes = vec![Ramble::new("King").with_kind(RambleKind::Theme)];

    let rr = RandomRamble::new().with_themes(themes);

    assert_eq!(
        rr,
        RandomRamble {
            rambles: vec![Ramble {
                kind: RambleKind::Theme,
                value: "King",
                file: None
            },],
            _rambles: HashMap::new(),
            template: None
        }
    );
}

#[test]
fn init_with_theme_from_string() {
    let theme = "Toto";

    let rr = RandomRamble::new().with_theme(theme.into());

    assert_eq!(
        rr,
        RandomRamble {
            rambles: vec![Ramble {
                kind: RambleKind::Theme,
                value: "Toto",
                file: None
            },],
            _rambles: HashMap::new(),
            template: None
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
            template: Some("A {{adj}} for {{theme}}")
        }
    );
}
