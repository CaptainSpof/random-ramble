#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;
    use random_ramble::refactor::{Ramble, RambleKind, RandomRamble};

    #[test]
    fn template_replace() {
        let adj = "Clever";
        let theme = "Toto";

        let r = RandomRamble::new()
            .with_template("A {{ adj | rr | lower }} {{ theme | rr }}")
            .with_adj(adj.into())
            .with_theme(theme.into());

        assert_eq!(r.to_string(), "A clever Toto".to_string());
    }

    #[test]
    fn template_replace_empty_map() {
        let r = RandomRamble::new()
            .with_adjs(vec![])
            .with_themes(vec![])
            .with_template("Nothing {{ theme | rr }}to{{ adj | rr }} see here.");

        assert_eq!(r.to_string(), "Nothing to see here.".to_string());
    }

    #[test]
    fn template_replace_default() {
        let adj = "Clever";
        let theme = "Toto";

        let r = RandomRamble::new()
            .with_adj(adj.into())
            .with_theme(theme.into());

        assert_eq!(r.to_string(), "Clever Toto".to_string());
    }

    #[test]
    fn template_replace_default_vec() {
        let adjs = vec!["Clever", "Stupid"];
        let themes = vec!["Titi", "Fifi"];

        let r = RandomRamble::new().with_adjs(adjs).with_themes(themes);

        // TODO: find better way to test randomness
        println!("{}", r.to_string());
        assert_eq!(r.to_string().len(), "Clever Titi".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec() {
        let adjs = vec!["Clever", "Stupid"];

        let emojis = vec!["🦀", "🐕"];

        let r = RandomRamble::new()
            .with_adjs(adjs)
            .with_others("emoji", emojis)
            .with_template("{{ adj | rr }} {{ emoji | rr }}");

        // TODO: find better way to test randomness
        // assert_eq!(r.to_string(), "Clever 🦀");
        assert_eq!(r.to_string().len(), "Clever 🦀".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec_with_ramble() {
        let en = vec!["Clever".into(), "Stupid".into()];

        let adjs = Ramble {
            category: Some("en".into()),
            values: en,
        };

        let emojis = vec!["🦀", "🐕"];

        let r = RandomRamble::new()
            .with_ramble(RambleKind::Adjective, adjs)
            .with_others("emoji", emojis)
            .with_template("{{ adj | rr }} {{ emoji | rr }}");

        // TODO: find better way to test randomness
        // assert_eq!(r.to_string(), "Clever 🦀");
        assert_eq!(r.to_string().len(), "Clever 🦀".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec_with_category() {
        let en = vec!["Clever".into(), "Stupid".into()];

        let adjs = Ramble {
            category: Some("en".into()),
            values: en,
        };

        let emojis = vec!["🦀", "🐕"];

        let r = RandomRamble::new()
            .with_ramble(RambleKind::Adjective, adjs)
            .with_others("emoji", emojis)
            .with_template("{{ adj | rr(c='en') }} {{ emoji | rr }}");

        // TODO: find better way to test randomness
        // assert_eq!(r.to_string(), "Clever 🦀");
        assert_eq!(r.to_string().len(), "Clever 🦀".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec_with_categories() {
        let en = vec!["Clever".into(), "Stupid".into()];
        let fr = vec!["Malin".into(), "Idiot".into()];

        let en_adjs = Ramble {
            category: Some("en".into()),
            values: en,
        };
        let fr_adjs = Ramble {
            category: Some("fr".into()),
            values: fr,
        };

        let emojis = vec!["🦀".into(), "🐕".into()];

        let r = RandomRamble::new()
            .with_rambles(RambleKind::Adjective, vec![en_adjs, fr_adjs])
            .with_others("emoji", emojis)
            .with_template("{{ adj | rr(c='fr') }} {{ emoji | rr }}");

        let r = r.to_string();
        let fr = vec!["Malin", "Idiot"];
        assert!(fr
            .iter()
            .any(|&a| a == r.split(' ').collect::<Vec<&str>>()[0]));

        // TODO: find better way to test randomness
        // assert_eq!(r.to_string(), "Idiot 🐕");
        assert_eq!(r.len(), "Malin 🦀".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec_with_categories_not_found() {
        let en = vec!["Clever".into(), "Stupid".into()];
        let fr = vec!["Malin".into(), "Idiot".into()];

        let en_adjs = Ramble {
            category: Some("en".into()),
            values: en,
        };
        let fr_adjs = Ramble {
            category: Some("fr".into()),
            values: fr,
        };

        let emojis = vec!["🦀", "🐕"];

        let r = RandomRamble::new()
            .with_rambles(RambleKind::Adjective, vec![en_adjs, fr_adjs])
            .with_others("emoji", emojis)
            .with_template("{{ adj | rr(c='pt') }} {{ emoji | rr }}");

        let r = r.to_string();
        assert_eq!(r, "???");
    }

    #[test]
    fn template_replace_themes_adjs_from_files() {
        let mut adj_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        adj_path.push("resources/tests/adjectives/");

        let mut theme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        theme_path.push("resources/tests/themes/");

        let rr = RandomRamble::new()
            .with_adjs_path(&adj_path)
            .expect("adjs not ok")
            .with_themes_path(&theme_path)
            .expect("themes not ok")
            .with_template("{{ adj | rr(c='en') }} {{ theme | rr(c='toto') }}")
            .to_string();

        // assert_eq!(
        //    rr,
        //     ""
        // );
        assert_eq!(rr.len(), 16);
    }
}
