#[macro_use]
extern crate log;

use env_logger::{Builder, WriteStyle};
use log::LevelFilter;

use rand::seq::SliceRandom;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;
use walkdir::WalkDir;

mod config;

use config::Config;

fn main() {
    let config: Config = Config::from_args();
    init_logger(config.verbose);
    debug!("config: {:#?}", config);

    let adjs: Vec<String> = WalkDir::new(&config.adjectives_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .filter(|file| {
            debug!("adj file: {:#?}", file);
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

    let themes: Vec<(String, String)> = WalkDir::new(&config.themes_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|metadata| metadata.file_type().is_file())
        .filter(|file| {
            debug!("theme file: {:#?}", file);
            match &config.themes {
                Some(sel_themes) if sel_themes.iter().any(|t| t.starts_with('!')) => {
                    let discarded_themes: Vec<_> = sel_themes
                        .iter()
                        .filter(|t| t.starts_with('!'))
                        .cloned()
                        .collect();
                    debug!("discarded_themes {:?}", discarded_themes);
                    let t = format!("!{}", file.file_name().to_str().unwrap());
                    !discarded_themes.contains(&t)
                }
                Some(sel_themes) => {
                    debug!("selected themes {:?}", sel_themes);
                    sel_themes.contains(&file.file_name().to_str().unwrap().to_string())
                }
                None => true,
            }
        })
        .flat_map(|f| {
            let file = std::fs::File::open(f.path()).expect("Unable to open file");
            let file_name = f.file_name().to_str().unwrap();
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| (l.expect("Could not parse line"), file_name.to_string()))
                .filter(|(l, _)| match config.starts_with {
                    Some(ref p) => l.to_lowercase().starts_with(&p.to_lowercase()),
                    None => true,
                })
                .collect::<Vec<(String, String)>>()
        })
        .collect();

    let adj_random_sel: Vec<_> = adjs
        .choose_multiple(&mut rand::thread_rng(), config.number)
        .collect();
    let themes_random_sel: Vec<_> = themes
        .choose_multiple(&mut rand::thread_rng(), config.number)
        .collect();
    let res: Vec<_> = adj_random_sel
        .iter()
        .zip(themes_random_sel.iter())
        .map(|(a, (t, p))| {
            if config.verbose >= 2 {
                format!("[{:^20}] {} {}", p, a, t)
            } else {
                format!("{} {}", a, t)
            }
        })
        .collect();

    for r in res {
        println!("{}", r);
    }
}

/// Init logger based on verbose value
/// # Arguments
///
/// * `verbose` - An integer representing the step of verbosity
///
/// TODO: Maybe allow logging to file ?
///
fn init_logger(verbose: u8) {
    let mut builder = Builder::new();

    match verbose {
        0 => builder
            .filter(None, LevelFilter::Info)
            .format_timestamp(None)
            .format_module_path(false),
        1 => builder.filter(None, LevelFilter::Info),
        2 => builder.filter(None, LevelFilter::Debug),
        _ => builder.filter(None, LevelFilter::Trace),
    };

    builder.write_style(WriteStyle::Always);
    builder.init();
}
