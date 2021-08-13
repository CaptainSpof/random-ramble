#[cfg(test)]
mod test {
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
        let en = vec!["Clever", "Stupid"];

        let adjs = Ramble { category: Some("en"), values: en };

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
        let en = vec!["Clever", "Stupid"];

        let adjs = Ramble { category: Some("en"), values: en };

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
        let en = vec!["Clever", "Stupid"];
        let fr = vec!["Malin", "Idiot"];

        let en_adjs = Ramble { category: Some("en"), values: en };
        let fr_adjs = Ramble { category: Some("fr"), values: fr };

        let emojis = vec!["🦀", "🐕"];

        let r = RandomRamble::new()
            .with_rambles(RambleKind::Adjective, vec![ en_adjs, fr_adjs])
            .with_others("emoji", emojis)
            .with_template("{{ adj | rr(c='fr') }} {{ emoji | rr }}");

        let fr = vec!["Malin", "Idiot"];
        assert!(fr.iter().any(|&a| a == r.to_string().split(' ').collect::<Vec<&str>>()[0]));

        // TODO: find better way to test randomness
        // assert_eq!(r.to_string(), "Idiot 🐕");
        assert_eq!(r.to_string().len(), "Malin 🦀".len());
    }

}
