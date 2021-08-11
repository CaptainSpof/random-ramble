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
        let adjs = vec!["Clever", "Stupid"]
            .into_iter()
            .map(|a| a.into())
            .collect();
        let themes = vec!["Titi", "Fifi"].into_iter().map(|t| t.into()).collect();

        let r = RandomRamble::new()
            .with_adjs(adjs)
            .with_themes(themes);

        // TODO: find better way to test randomness
        assert_eq!(r.to_string().len(), "Clever Titi".len());
    }

}
