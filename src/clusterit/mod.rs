use std::{path::PathBuf, sync::mpsc, thread};

use anyhow::Result;

mod config;

use config::Config;
use log::debug;
use ssh::Session;

pub struct Clusterit {
    config: Config,
}

impl Clusterit {
    pub fn from_file(path: &PathBuf) -> Result<Clusterit> {
        let config = Config::from_file(path)?;
        println!("{:?}", config);

        Ok(Clusterit { config })
    }

    pub fn setup(self) -> Result<()> {
        for (device_name, server) in self.config.servers.into_iter() {
            debug!("Connecting to {} ({})", device_name, server.ip);
            let (sender, receiver) = mpsc::channel();

            thread::spawn(move || {
                debug!("Spawned new thread");
                let mut session = Session::new().unwrap();
                session.set_host(&server.ip).unwrap();
                session.parse_config(None).unwrap();
                session.connect().unwrap();
                println!("asdf");
                session.userauth_password("asdf").unwrap();
                session.userauth_publickey_auto(Some("asdf")).unwrap();

                session.disconnect().unwrap();
            })
            .join()
            .unwrap();

            sender.send("asdf")?;
        }

        Ok(())
    }
}
