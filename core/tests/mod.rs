#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;
    use random_ramble::refactor::RandomRamble;

    #[test]
    fn template_replace() {
        let adj = "Clever";
        let theme = "Toto";

        let r = RandomRamble::new()
            .with_template("A {{ adj | rr | lower }} {{ theme | rr }}")
            .with_ramble("adj", adj)
            .with_ramble("theme", theme)
            .build()
            .expect("we gud");

        assert_eq!(r.to_string(), "A clever Toto".to_string());
    }

    #[test]
    fn template_replace_empty_map() {
        let r = RandomRamble::new()
            .with_rambles("adj", vec![])
            .with_rambles("theme", vec![])
            .with_template("Nothing {{ theme | rr }}to{{ adj | rr }} see here.")
            .build()
            .expect("we gud");

        assert_eq!(r.to_string(), "Nothing to see here.".to_string());
    }

    #[test]
    fn template_replace_default() {
        let adj = "Clever";
        let theme = "Toto";

        let r = RandomRamble::new()
            .with_ramble("adj", adj)
            .with_ramble("theme", theme)
            .build()
            .expect("we gud");

        assert_eq!(r.to_string(), "Clever Toto".to_string());
    }

    #[test]
    fn template_replace_default_with_multiple() {
        let adj = "Clever";
        let theme = "Toto";

        let r = RandomRamble::new()
            .with_ramble("adj", adj)
            .with_ramble("theme", theme)
            .build()
            .expect("we gud")
            .take(2);

        assert_eq!(r, vec!["Clever Toto", "Clever Toto"]);

        assert_eq!(r.len(), 2);
    }

    #[test]
    fn template_replace_default_vec() {
        let adjs = vec!["Clever", "Stupid"];
        let themes = vec!["Titi", "Fifi"];

        let r = RandomRamble::new()
            .with_rambles("adj", adjs)
            .with_rambles("theme", themes)
            .build()
            .expect("we gud");

        // TODO: find better way to test randomness
        println!("{}", r.to_string());
        assert_eq!(r.to_string().len(), "Clever Titi".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec() {
        let adjs = vec!["Clever", "Stupid"];

        let emojis = vec!["🦀", "🐕"];

        let r = RandomRamble::new()
            .with_rambles("adj", adjs)
            .with_rambles("emoji", emojis)
            .with_template("{{ adj | rr }} {{ emoji | rr }}")
            .build()
            .expect("we gud");

        // TODO: find better way to test randomness
        assert_eq!(r.to_string().len(), "Clever 🦀".len());
    }

    #[test]
    fn template_replace_custom_ramble_vec_multiple() {
        let adjs = vec!["Clever", "Stupid"];

        let emojis = vec!["🦀", "🐕", "🐈", "🐖", "🐄"];

        let r = RandomRamble::new()
            .with_rambles("adj", adjs)
            .with_rambles("emoji", emojis)
            .with_template("{{ adj | rr }} {{ emoji | rr }}")
            .build()
            .expect("we gud")
            .take(15);

        // TODO: find better way to test randomness
        assert_eq!(r.len(), 15);
    }

    #[test]
    fn template_replace_themes_adjs_from_files() {
        let mut adj_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        adj_path.push("resources/tests/adjectives/");

        let mut theme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        theme_path.push("resources/tests/themes/");

        let rr = RandomRamble::new()
            .with_rambles_path("adj", &adj_path)
            .expect("adjs not ok")
            .with_rambles_path("theme", &theme_path)
            .expect("themes not ok")
            .with_template("{{ adj | rr(c='en') }} {{ theme | rr(c='toto') }}")
            .build()
            .expect("we gud")
            .to_string();

        // assert_eq!(
        //    rr,
        //     ""
        // );
        assert_eq!(rr.len(), 16);
    }

    #[test]
    fn template_filter_by_categories_and_starting_pattern() {
        let mut adj_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        adj_path.push("resources/tests/adjectives/");

        let mut theme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        theme_path.push("resources/tests/themes/");

        let rr = RandomRamble::new()
            .with_rambles_path("adj", &adj_path)
            .expect("adjs not ok")
            .with_rambles_path("theme", &theme_path)
            .expect("themes not ok")
            .with_template("{{ adj | rr(c='en', starts_with='a') }} {{ theme | rr(c='foobar', starts_with='b') }}")
            .build()
            .expect("we gud")
            .to_string();

        assert_eq!(rr, "Adventurous bar");
        assert_eq!(rr.len(), 15);
    }
}
