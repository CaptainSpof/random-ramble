#[macro_use]
extern crate log;

use env_logger::{Builder, WriteStyle};
use log::LevelFilter;

use clap::Parser;

mod cmds;
mod config;

use config::{Command, Config};
use random_ramble::refactor::RandomRamble as _RandomRamble;
use random_ramble::{RambleError, RandomRamble};

fn main() -> Result<(), RambleError> {
    let config: Config = Config::parse();
    init_logger(config.verbose);
    debug!("config: {:#?}", config);

    let themes = config.themes.iter().map(AsRef::as_ref).collect();
    let adjs = config.adjs.iter().map(AsRef::as_ref).collect();

    if config.legacy {
        let rr = match RandomRamble::new(&config.adjectives_path, adjs, &config.themes_path, themes)
        {
            Ok(rr) => rr,
            Err(e) => {
                error!("Crote, une erreur: {}", e);
                std::process::exit(1);
            }
        };

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
                let ramble = rr.randomize(
                    config.pattern.as_deref(),
                    config.number,
                    Some(&config.template),
                    config.verbose >= 1,
                );

                match ramble {
                    Ok(ramble) => {
                        for r in ramble {
                            println!("{}", r);
                        }
                    }
                    Err(e) => {
                        eprint!("Zut ! {}", e);
                        std::process::exit(1);
                    }
                }
            }
        };
    } else {
        let templates = config.templates.iter().map(AsRef::as_ref).collect();
        // TODO: add filter to rambles
        let rr = _RandomRamble::new()
            .with_filter(config.pattern.as_deref())
            .with_rambles("adj", adjs)
            .with_rambles("theme", themes)
            .with_rambles_path("adj", &config.adjectives_path)?
            .with_rambles_path("theme", &config.themes_path)?
            .with_templates(templates)
            .build()?;

        for r in rr.take(config.number) {
            println!("{}", r);
        }
    };
    Ok(())
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
