#[macro_use]
extern crate log;

use env_logger::{Builder, WriteStyle};
use log::LevelFilter;

use structopt::StructOpt;

mod config;
mod cmds;

use config::{ Config, Command };
use random_ramble::{get_random_ramble, get_random_ramble_with_provenance};

fn main() {
    let config: Config = Config::from_args();
    init_logger(config.verbose);
    debug!("config: {:#?}", config);

    match config.cmd {
        Some(Command::Add(c)) => cmds::add(&config.themes_path, c.theme, c.entries),
        Some(Command::Delete(_)) => cmds::delete(),
        None => {

            let res = match config.verbose {
                v if v < 1 => get_random_ramble(
                    &config.adjectives_path,
                    config.adjectives,
                    &config.themes_path,
                    config.themes,
                    config.pattern.as_deref(),
                    config.number,
                ),
                _ => get_random_ramble_with_provenance(
                    &config.adjectives_path,
                    config.adjectives,
                    &config.themes_path,
                    config.themes,
                    config.pattern.as_deref(),
                    config.number,
                ),
            };

            for r in res {
                println!("{}", r);
            }
        }
    };
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
