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

    #[structopt(
        short = "s",
        long = "state",
        help = "Path to state file",
        default_value = "state.json"
    )]
    state: PathBuf,

    #[structopt(long = "log-level", global = true, default_value = "warn", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        #[structopt(help = "Destination to device (ip, hostname)")]
        destination: String,
    },
    Devices {},
    Sync {},
}

fn main() {
    let opt = Opt::from_args();
    let loglevel = opt.loglevel;
    let statefile = opt.state;

    fern::Dispatch::new()
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let clusterit = Clusterit::from_file(&statefile).expect("Failed to load clusterit");

    let res = match opt.cmd {
        Command::Sync {} => clusterit.sync(),
        Command::Add { destination } => clusterit.add_device(&destination),
        _ => todo!(),
    };

    if let Err(e) = res {
        error!("{e}");
    }
}
