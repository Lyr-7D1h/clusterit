use std::{net::IpAddr, path::PathBuf, process::exit};

use k3main::k3main::K3main;
use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "k3main",
    author = "Lyr 7d1h",
    about = "A tool for settings up and managing a k3 cluster."
)]

struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(long = "log-level", global = true, default_value = "debug", possible_values(&["debug", "info", "warn", "error"]))]
    loglevel: LevelFilter, // TODO change to verbosity (-v)
}

#[derive(StructOpt, Debug)]
enum Command {
    Init {
        #[structopt(long = "ssh-pub-key", help = "Run only a specific step")]
        ssh_pub_key: PathBuf,
    },
    Setup {
        #[structopt(
            long = "ip",
            help = "The ip address of the machine you want to setup will connect using known default ssh credentials."
        )]
        ip: IpAddr,

        #[structopt(long = "step", help = "Run only a specific step")]
        step: Option<u8>,
    },
}
fn main() {
    let opt = Opt::from_args();

    SimpleLogger::new().with_level(opt.loglevel).init().unwrap();

    match opt.command {
        Command::Init { ssh_pub_key } => {
            if let Err(e) = K3main::init(&ssh_pub_key) {
                error!("{}", e);
                exit(1);
            };
        }
        Command::Setup { ip, step } => {
            let k3main = match K3main::load() {
                Ok(k3main) => k3main,
                Err(e) => {
                    error!("{}", e);
                    exit(1);
                }
            };

            if let Err(e) = k3main.setup(ip, step) {
                error!("Setup failed: {}", e);
                exit(1)
            }
        }
    };
}
