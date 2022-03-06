use log::{error, LevelFilter};
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
    #[structopt(subcommand)]
    cmd: Command,

    #[structopt(long = "log-level", global = true, default_value = "warn", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(help = "asdf")]
    SetupPubAuth { destination: String },
    Apply {
        #[structopt(
            short = "c",
            long = "config",
            help = "Path to config file",
            default_value = "config.toml"
        )]
        config: PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();

    fern::Dispatch::new()
        .level(opt.loglevel)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let res = match opt.cmd {
        Command::SetupPubAuth { destination } => Clusterit::setup_pub_auth(&destination),
        Command::Apply { config } => Clusterit::from_file(&config)
            .expect("Failed to load clusterit")
            .execute(),
    };

    if let Err(e) = res {
        error!("{e}");
    }
}
