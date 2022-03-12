use std::path::PathBuf;

pub mod config;
mod connection;
mod executer;

use config::Config;
use connection::Connection;

mod error;
use error::ClusteritError;

pub struct Clusterit {
    config: Config,
}

impl Clusterit {
    pub fn from_file(path: &PathBuf) -> Result<Clusterit, ClusteritError> {
        let config = Config::from_file(path)?;
        println!("{:?}", config);

        Ok(Clusterit { config })
    }

    pub fn execute(self) -> Result<(), ClusteritError> {
        // TODO pass public and private key from state
        let connection = Connection::connect_to_destination(&self.config.destination, None, None)?;
        todo!()
    }
}
