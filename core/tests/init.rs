use std::path::PathBuf;

use random_ramble::RandomRamble;

#[test]
fn init_default() {
    let rr = RandomRamble::default();

    assert_eq!(rr, RandomRamble{
        rambles: vec![],
        template: None
    });
}

#[test]
fn init_with_adjs() {

    let adjs = vec![
        RambleBuilder_::default()
            .kind(RambleKind_::Adjective)
            .value("Adjective".to_string())
            .build()
    ];

    let rr = RandomRamble::new()
        .with_adjs(adjs);

    assert_eq!(rr, RandomRamble{
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Adjective,
                value: "Adjective".to_string(),
                file: None
            },
        ],
        template: None
    });
}

#[test]
fn init_with_adj_from_string() {
    let adj = "Pretty".to_string();

    let rr = RandomRamble::new()
        .with_adj(adj.into());

    assert_eq!(rr, RandomRamble{
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Adjective,
                value: "Pretty".to_string(),
                file: None
            },
        ],
        template: None
    });
}

#[test]
fn init_with_adjs_from_string() {
    let adj1 = "Kind".to_string();
    let adj2 = "Ruthless".to_string();

    let rr = RandomRamble::new()
        .with_adjs(vec![adj1.into(), adj2.into()]);

    assert_eq!(rr, RandomRamble{
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Adjective,
                value: "Kind".to_string(),
                file: None
            },
            Ramble_ {
                kind: RambleKind_::Adjective,
                value: "Ruthless".to_string(),
                file: None
            },
        ],
        template: None
    });
}


#[test]
fn init_with_adjs_from_path() {

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/tests/adjectives/");

    let rr = RandomRamble::new()
        .with_adjs_path(&path);

    assert_eq!(rr, RandomRamble {
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Adjective,
                value: "Ugly".to_string(),
                file: Some(File {
                    name: "test2".to_string(),
                    path: format!("{}test2", path.clone().into_os_string().into_string().expect("🤷"))
                })
            },
            Ramble_ {
                kind: RambleKind_::Adjective,
                value: "Pretty".to_string(),
                file: Some(File {
                    name: "test1".to_string(),
                    path: format!("{}test1", path.clone().into_os_string().into_string().expect("🤷"))
                })
            },
        ],
        template: None
    });
}

#[test]
fn init_with_themes() {

    let themes = vec![
        RambleBuilder_::default()
            .kind(RambleKind_::Theme)
            .value("Theme".to_string())
            .build()
    ];

    let rr = RandomRamble::new()
        .with_themes(themes);

    assert_eq!(rr, RandomRamble{
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Theme,
                value: "Theme".to_string(),
                file: None
            },
        ],
        template: None
    });
}

#[test]
fn init_with_theme_from_string() {
    let theme = "Toto".to_string();

    let rr = RandomRamble::new()
        .with_theme(theme.into());

    assert_eq!(rr, RandomRamble{
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Theme,
                value: "Toto".to_string(),
                file: None
            },
        ],
        template: None
    });
}

#[test]
fn init_with_themes_from_path() {

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/tests/themes/");

    let rr = RandomRamble::new()
        .with_themes_path(&path);

    assert_eq!(rr, RandomRamble {
        rambles: vec![
            Ramble_ {
                kind: RambleKind_::Theme,
                value: "Titi".to_string(),
                file: Some(File {
                    name: "test2".to_string(),
                    path: format!("{}test2", path.clone().into_os_string().into_string().expect("🤷"))
                })
            },
            Ramble_ {
                kind: RambleKind_::Theme,
                value: "Toto".to_string(),
                file: Some(File {
                    name: "test1".to_string(),
                    path: format!("{}test1", path.clone().into_os_string().into_string().expect("🤷"))
                })
            },
        ],
        template: None
    });
}

#[test]
fn init_with_template() {

    let rr = RandomRamble::new()
        .with_template("A {{adj}} for {{theme}}".to_string());

    assert_eq!(rr, RandomRamble{
        rambles: vec![],
        template: Some("A {{adj}} for {{theme}}".to_string())
    });
}
