use structopt::clap::AppSettings;
use structopt::StructOpt;

use std::path::PathBuf;

use std::io::{self, Read};

#[derive(StructOpt, Debug)]
#[structopt(author, name = "random-ramble", about = "A simple random words generator", global_settings(&[AppSettings::ColoredHelp]))]
pub struct Config {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(
        short,
        long,
        parse(from_occurrences),
        long_help = "-v:\t\tINFO|WARN|ERROR\n-vv:\tINFO|WARN|ERROR|DEBUG\n-vvv:\tINFO|WARN|ERROR|DEBUG|TRACE"
    )]
    pub verbose: u8,

    /// The length of the list to be returned
    #[structopt(short, env = "RR_NB_RESULT", default_value = "10")]
    pub number: usize,

    /// Path to the themes files
    #[structopt(long, env = "RR_THEMES_PATH", default_value = "./dict/themes")]
    pub themes_path: PathBuf,

    /// A list of themes to be chosen from
    ///
    /// Themes preceded by '!' will be excluded
    #[structopt(short, long)]
    pub themes: Option<Vec<String>>,

    /// Path to the adjectives files
    #[structopt(long, env = "RR_ADJS_PATH", default_value = "./dict/adjectives")]
    pub adjectives_path: PathBuf,

    /// A list of adjectives to be chosen from
    #[structopt(short, long)]
    pub adjectives: Option<Vec<String>>,

    /// Provide a template from which to generate words
    #[structopt(long)]
    pub template: Option<String>,

    /// The pattern to start with
    pub pattern: Option<String>,

    /// cmd
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Command")]
pub enum Command {
    #[structopt(name = "add")]
    /// Add entries to a theme, or create a new theme
    Add(Edit),
    /// Delete entries from a theme, or create a new theme
    #[structopt(visible_aliases = &["remove", "del"])]
    Delete(Edit),
}

#[derive(StructOpt, Debug)]
pub struct Edit {
    /// Provide a theme
    pub theme: String,

    /// Provide a list of entries
    ///
    /// Will attempt to read from stdin
    entries: Vec<String>,

    /// Work against adjectif
    #[structopt(short)]
    pub adjs: bool,
}

impl Edit {
    pub fn entries(&self) -> Vec<String> {
        match &self.entries {
            entries if entries.is_empty() => {
                let mut buffer = String::new();
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                handle.read_to_string(&mut buffer).unwrap();

                let entries: Vec<String> = buffer.split_whitespace().map(String::from).collect();
                entries
            }
            entries => entries.clone(),
        }
    }
}
