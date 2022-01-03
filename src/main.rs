use env_logger::Builder;
use log::LevelFilter;
use std::path::PathBuf;
use structopt::StructOpt;

use clusterit::Clusterit;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "clusterit",
    author = "Lyr 7d1h",
    about = "A tool for settings up and managing a k3 cluster."
)]
struct Opt {
    #[structopt(
        short = "c",
        help = "Run only a specific step",
        default_value = "config.toml"
    )]
    config: PathBuf,

    #[structopt(long = "log-level", global = true, default_value = "debug", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter,
}

fn main() {
    let opt = Opt::from_args();

    Builder::new().filter(None, opt.loglevel).init();

    let clusterit = Clusterit::from_file(&opt.config).expect("Failed to load clusterit");

    clusterit.setup().expect("Setup failed")
}
