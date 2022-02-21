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
    #[structopt(long = "log-level", global = true, default_value = "warn", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter,

    #[structopt(subcommand)]
    cmd: Command,
}

enum Command {
    SetupPubAuth {
        destination: String,
    },
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

    Builder::new().filter(None, opt.loglevel).init();

    match opt {
        Opt::SetupPubAuth { destination } => Clusterit::setup_pub_auth(destination),
        Opt::Apply { config, loglevel } => Clusterit::from_file(config)
            .expect("Failed to load clusterit")
            .execute()
            .expect("Execute failed"),
    }
}
