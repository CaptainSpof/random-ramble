use structopt::StructOpt;
use walkdir::WalkDir;

mod config;

use config::Config;

use rand::seq::SliceRandom;
use std::{
    env,
    io::{prelude::*, BufReader},
};

fn main() {
    let config: Config = Config::from_args();

    println!("config: {:#?}", config);

    let home = env::var("HOME").expect("Unable to get HOME environment variable");
    let adjs: Vec<String> = WalkDir::new(format!("{}/Projects/Rust/random-ramble/dict/adjectives", home))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let buf = BufReader::new(file);
            buf.lines()
               .map(|l| l.expect("Could not parse line"))
               .collect::<Vec<String>>()
        })
        .collect();


    let themes: Vec<String> = WalkDir::new(format!("{}/Projects/Rust/random-ramble/dict/themes", home))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let buf = BufReader::new(file);
            buf.lines()
               .map(|l| l.expect("Could not parse line"))
               .collect::<Vec<String>>()
        })
        .collect();

    let adj_random_sel: Vec<_> = adjs.choose_multiple(&mut rand::thread_rng(), config.list_length as usize).collect();
    let themes_random_sel: Vec<_> = themes.choose_multiple(&mut rand::thread_rng(), config.list_length as usize).collect();
    println!("adjs: {:#?}", adj_random_sel);
    println!("themes: {:#?}", themes_random_sel);

    let res: Vec<_> = adj_random_sel.iter().zip(themes_random_sel.iter()).collect();
    let res: Vec<String> = res.iter().map(|(a, t)| format!("{} {}", a, t)).collect();

    for r in res {
        println!("{}", r);
    }

}
