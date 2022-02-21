use std::path::PathBuf;

pub mod config;
mod connection;
mod executer;

use config::Config;
use connection::Connection;
use connection::ConnectionError;
use executer::Executer;

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

    pub fn setup_pub_auth(destination: String) -> Result<(), ClusteritError> {
        let connection = Connection::connect_to_destination(destination)?;
    }

    pub fn execute(self) -> Result<(), ClusteritError> {
        // let connection = Connection::connect(&mut self, user, password)
        todo!()
    }
}
