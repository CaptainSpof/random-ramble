#[cfg(test)]
mod test {
    use random_ramble::refactor::RandomRamble;

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

        let r = RandomRamble::new()
            .with_adjs(adjs)
            .with_themes(themes);

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

        dbg!(&r);

        let r = match r.replace() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{:?}", e);
                "oups".into()
            }
        };
        // TODO: find better way to test randomness
        // assert_eq!(r, "Clever 🦀");
        assert_eq!(r.to_string().len(), "Clever 🦀".len());
    }
}
