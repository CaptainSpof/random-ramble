use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, name = "random-ramble", global_settings(&[AppSettings::ColoredHelp]))]
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
    #[structopt(short, long, default_value = "10")]
    pub list_length: u8,

    /// A list of themes to be choose from
    #[structopt(short, long)]
    pub themes: Option<Vec<String>>,

    /// A list of adjectives to be choose from
    #[structopt(short, long, default_value = "adjectives_en")]
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
    /// Add entres to a theme, or create a new theme
    Add(Edit),
    /// Generates env from known_hosts
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
