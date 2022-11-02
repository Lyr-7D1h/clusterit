use std::path::PathBuf;

pub mod state;
use error::ClusteritErrorKind;
use state::{Device, State};

mod connection;
pub use connection::Connection;
pub use connection::Destination;

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

    pub fn add_device(&self, destination: &Destination) -> Result<(), ClusteritError> {
        if self.state.exists(destination) {
            return Err(ClusteritError::new(
                ClusteritErrorKind::Generic,
                "destination already exists",
            ));
        }
        let connection = Connection::connect_interactive(destination)?;
        todo!()
    }

    pub fn devices(&self) -> &Vec<Device> {
        return &self.state.devices;
    }
}
