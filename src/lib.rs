use std::path::PathBuf;

pub mod state;
use state::{State, Device};

mod connection;
use connection::Connection;

mod executer;

mod error;
use error::ClusteritError;

pub struct Clusterit {
    pub state: State,
}

impl Clusterit {
    pub fn from_file(path: &PathBuf) -> Result<Clusterit, ClusteritError> {
        let state = State::from_file(path)?;

        Ok(Clusterit { state })
    }

    pub fn sync(&self) -> Result<(), ClusteritError> {
        todo!()
    }

    pub fn add_device(&self, destination: &String) -> Result<(), ClusteritError> {
        let connection = Connection::connect_to_destination(&destination, None, None)?;
        todo!()
    }

    pub fn devices(&self) -> &Vec<Device> {
        return &self.state.devices
    }
}
