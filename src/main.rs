use structopt::StructOpt;

mod config;

use config::Config;

fn main() {
    let config: Config = Config::from_args();

    println!("config: {:#?}", config);
}
