use std::{net::IpAddr, path::PathBuf, process::exit};

use clusterit::clusterit::clusterit;
use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "clusterit",
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
            if let Err(e) = clusterit::init(&ssh_pub_key) {
                error!("{}", e);
                exit(1);
            };
        }
        Command::Setup { ip, step } => {
            let clusterit = match clusterit::load() {
                Ok(clusterit) => clusterit,
                Err(e) => {
                    error!("{}", e);
                    exit(1);
                }
            };

            if let Err(e) = clusterit.setup(ip, step) {
                error!("Setup failed: {}", e);
                exit(1)
            }
        }
    };
}
