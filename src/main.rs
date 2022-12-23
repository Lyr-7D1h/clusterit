use log::{error, LevelFilter};
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

use clusterit::{Clusterit, Destination};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "clusterit",
    author = "Lyr 7d1h",
    about = "A tool for settings up and managing servers."
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

    #[structopt(short = "l", long = "log-level", global = true, default_value = "warn", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter,
}

#[derive(StructOpt, Debug)]
enum Command {
    Apply {
        #[structopt(help = "The clusterit module you want to run on the machine")]
        module: PathBuf,

        #[structopt(help = "Destination to device (ip, hostname)")]
        destination: Destination,
    },
}

fn main() {
    let opt = Opt::from_args();
    let loglevel = opt.loglevel;

    fern::Dispatch::new()
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let mut clusterit = Clusterit::from_file(&opt.state).expect("Failed to load clusterit");

    let res = match opt.cmd {
        Command::Apply {
            module,
            destination,
        } => clusterit.apply(&module, destination, vec![]),
        _ => todo!(),
    };

    if let Err(e) = res {
        error!("{e}");
    }
}
