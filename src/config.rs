use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(StructOpt, Debug)]
#[structopt(author, name = "random-ramble", global_settings(&[AppSettings::ColoredHelp]))]
pub struct Config {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences), long_help = "-v:\t\tINFO|WARN|ERROR\n-vv:\tINFO|WARN|ERROR|DEBUG\n-vvv:\tINFO|WARN|ERROR|DEBUG|TRACE")]
    verbose: u8,

    /// The length of the list to be returned
    #[structopt(short, long, default_value = "10")]
    pub list_length: u8,

    /// A list of themes to be choose from
    #[structopt(short, long)]
    pub themes: Vec<String>,

    pub starts_with: Option<String>,
}
