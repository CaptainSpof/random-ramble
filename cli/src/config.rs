use clap::Parser;

use std::path::PathBuf;

use std::io::{self, Read};

#[derive(Parser, Debug)]
#[clap(
    author,
    name = "random-ramble",
    about = "A simple random words generator",
    version
)]
pub struct Config {
    /// Verbose mode (-v, -vv, -vvv)
    #[clap(
        short,
        long,
        action = clap::ArgAction::Count,
        long_help = "-v:\t\tINFO|WARN|ERROR\n-vv:\tINFO|WARN|ERROR|DEBUG\n-vvv:\tINFO|WARN|ERROR|DEBUG|TRACE"
    )]
    pub verbose: u8,

    /// The length of the list to be returned
    #[clap(short, env = "RR_NB_RESULT", default_value = "10", action)]
    pub number: usize,

    /// Path to the themes files
    // TODO: respect XDG convention
    #[clap(long, env = "RR_THEMES_PATH", default_value = "./dict/themes", action)]
    pub themes_path: PathBuf,

    /// A list of themes to be chosen from
    ///
    /// Themes preceded by '!' will be excluded
    #[clap(short, long, action)]
    pub themes: Vec<String>,

    /// Path to the adjectives files
    // TODO: respect XDG convention
    #[clap(long, env = "RR_ADJS_PATH", default_value = "./dict/adjectives", action)]
    pub adjectives_path: PathBuf,

    /// A list of adjectives to be chosen from
    #[clap(short, long, action)]
    pub adjs: Vec<String>,

    /// Provide a template from which to generate words
    // FIXME: use custom rr filter
    #[clap(long, default_value = "{{ adj }} {{ theme }}", action)]
    #[deprecated(note = "Replaced with `templates`")]
    pub template: String,

    /// Provide templates from which to generate words
    #[clap(short = 'T', long, default_value = "{{ adj | rr }} {{ theme | rr }}", action)]
    pub templates: Vec<String>,

    /// try the legacy version
    #[clap(short, long, action)]
    pub legacy: bool,

    /// The pattern to start with
    #[clap(action)]
    pub pattern: Option<String>,

    /// cmd
    #[clap(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Parser, Debug)]
#[clap(name = "Command")]
pub enum Command {
    #[clap(name = "add")]
    /// Add entries to a theme, or create a new theme
    Add(Edit),
    /// Delete entries from a theme
    #[clap(visible_aliases = &["remove", "del"])]
    Delete(Edit),
}

#[derive(Parser, Debug)]
pub struct Edit {
    /// Provide a theme
    #[clap(action)]
    pub theme: String,

    /// Provide a list of entries
    ///
    /// Will attempt to read from stdin
    #[clap(action)]
    entries: Vec<String>,

    /// Work against adjectif
    #[clap(short, action)]
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
