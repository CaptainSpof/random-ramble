use structopt::clap::AppSettings;
use structopt::StructOpt;

use std::path::PathBuf;

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
    verbose: u8,

    /// The length of the list to be returned
    #[structopt(short, long, env = "RR_LIST_LENGTH", default_value = "10")]
    pub list_length: usize,

    /// Path to the themes files
    #[structopt(long, env = "RR_THEMES_PATH", default_value = "./dict/themes")]
    pub themes_path: PathBuf,

    /// A list of themes to be chosen from
    #[structopt(short, long)]
    pub themes: Option<Vec<String>>,

    /// Path to the adjectives files
    #[structopt(long, env = "RR_ADJS_PATH", default_value = "./dict/adjectives")]
    pub adjectives_path: PathBuf,

    /// A list of adjectives to be chosen from
    #[structopt(long, default_value = "adjectives_en")]
    pub adjectives: Vec<String>,

    pub starts_with: Option<String>,

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
    Delete(Edit),
}

#[derive(StructOpt, Debug)]
#[structopt(author, name = "Add", global_settings(&[AppSettings::ColoredHelp]))]
pub struct Edit {
    /// Provide a theme
    pub theme: String,

    /// Provide a list of entries
    pub entries: Vec<String>,
}
