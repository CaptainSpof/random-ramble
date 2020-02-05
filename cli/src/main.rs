#[macro_use]
extern crate log;

use env_logger::{Builder, WriteStyle};
use log::LevelFilter;

use structopt::StructOpt;

mod cmds;
mod config;

use config::{Command, Config};
use random_ramble::RandomRamble;

fn main() {
    let config: Config = Config::from_args();
    init_logger(config.verbose);
    debug!("config: {:#?}", config);

    let rr = RandomRamble::new(
        &config.adjectives_path,
        config.adjectives.clone(),
        &config.themes_path,
        config.themes.clone(),
    );

    match config.cmd {
        Some(Command::Add(c)) => {
            if c.adjs {
                cmds::add(&config.adjectives_path, &c.theme, c.entries())
            } else {
                cmds::add(&config.themes_path, &c.theme, c.entries());
            }
        }
        Some(Command::Delete(c)) => {
            if c.adjs {
                cmds::delete(&config.adjectives_path, &c.theme, c.entries())
            } else {
                cmds::delete(&config.themes_path, &c.theme, c.entries());
            }
        }
        None => {
            let res = match config.verbose {
                v if v < 1 => rr.randomize(config.pattern.as_deref(), config.number),
                _ => rr.randomize_with_details(config.pattern.as_deref(), config.number),
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
