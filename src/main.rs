use structopt::StructOpt;
use walkdir::WalkDir;

mod config;

use config::Config;

use rand::seq::SliceRandom;
use std::io::{prelude::*, BufReader};

fn main() {
    let config: Config = Config::from_args();
    println!("{:#?}", config);

    let adjs: Vec<String> = WalkDir::new(&config.adjectives_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .filter(|file| {
            config
                .adjectives
                .contains(&file.file_name().to_str().unwrap().to_string())
        })
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .filter(|l| match config.starts_with {
                    Some(ref p) => l.to_lowercase().starts_with(&p.to_lowercase()),
                    None => true,
                })
                .collect::<Vec<String>>()
        })
        .collect();

    let themes: Vec<String> = WalkDir::new(&config.themes_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .filter(|file| match &config.themes {
            Some(themes) => themes.contains(&file.file_name().to_str().unwrap().to_string()),
            None => true,
        })
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .filter(|l| match config.starts_with {
                    Some(ref p) => l.to_lowercase().starts_with(&p.to_lowercase()),
                    None => true,
                })
                .collect::<Vec<String>>()
        })
        .collect();

    let adj_random_sel: Vec<_> = adjs
        .choose_multiple(&mut rand::thread_rng(), config.list_length)
        .collect();
    let themes_random_sel: Vec<_> = themes
        .choose_multiple(&mut rand::thread_rng(), config.list_length)
        .collect();
    let res: Vec<_> = adj_random_sel
        .iter()
        .zip(themes_random_sel.iter())
        .map(|(a, t)| format!("{} {}", a, t))
        .collect();

    for r in res {
        println!("{}", r);
    }
}
