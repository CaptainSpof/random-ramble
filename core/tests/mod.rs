use random_ramble::refactor::{Ramble, RambleKind, RandomRamble};

#[test]
fn template_replace() {
    let adj = "Clever";
    let theme = "Toto";

    let r = RandomRamble::new()
        .with_template("A {{adj | lower }} {{theme}}")
        .with_adj(adj.into())
        .with_theme(theme.into())
        .replace();

    assert_eq!(r.unwrap(), "A clever Toto".to_string());
}
