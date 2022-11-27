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

    // #[structopt(
    //     short = "s",
    //     long = "state",
    //     help = "Path to state file",
    //     default_value = "state.json"
    // )]
    // state: PathBuf,
    #[structopt(long = "log-level", global = true, default_value = "warn", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter,
}

#[derive(StructOpt, Debug)]
enum Command {
    Apply {
        #[structopt(help = "The clusterit module you want to run on the machine")]
        module: String,

        #[structopt(help = "Destination to device (ip, hostname)")]
        destination: String,
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

    let clusterit = Clusterit::from_file(&statefile).expect("Failed to load clusterit");

    let res = match opt.cmd {
        Command::Sync {} => clusterit.sync(),
        Command::Add { destination } => {
            let destination =
                Destination::from_str(&destination).expect("Failed to parse destination");
            clusterit.add_device(&destination)
        }
        _ => todo!(),
    };

    if let Err(e) = res {
        error!("{e}");
    }
}
