use std::{net::IpAddr, process::exit};

use k3main::k3main::{setup::Device, K3main};
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
    Init,
    Flash,
    Setup {
        #[structopt(
            long = "ip",
            help = "The ip address of the machine you want to setup will connect using known default ssh credentials."
        )]
        ip: IpAddr,

        #[structopt(long = "device", possible_values(&["raspberrypi", "odroidmc1", "odroidhc4"]))]
        device: Option<Device>,
    },
}
fn main() {
    let opt = Opt::from_args();

    SimpleLogger::new().with_level(opt.loglevel).init().unwrap();

    let k3main = match K3main::new() {
        Ok(k3main) => k3main,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    match opt.command {
        Command::Init => {}
        Command::Flash => {}
        Command::Setup { ip, device } => {
            if let Err(e) = k3main.setup(ip, device) {
                error!("Setup failed: {}", e);
                exit(1)
            }
        }
    };
}
