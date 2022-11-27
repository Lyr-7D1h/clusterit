use std::path::Path;
use std::path::PathBuf;

pub mod state;
use executer::Executer;
use log::error;
use parser::Module;
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

    pub fn apply(
        &self,
        module_path: &Path,
        destination: &Destination,
        arguments: Vec<String>,
    ) -> Result<(), ClusteritError> {
        let module = Module::new(module_path, arguments)?;

        let device = self.state.get_device(destination);
        let connection = if let Some(device) = device {
            match Connection::connect(destination, device.authentication) {
                Ok(c) => c,
                Err(_) => {
                    error!("Could not log into device with saved credentials.");
                    Connection::connect_interactive(destination)?
                }
            }
        } else {
            Connection::connect_interactive(destination)?
        };

        let executer = Executer::from_state(connection, self.state);

        return Ok(());
    }

    pub fn devices(&self) -> &Vec<Device> {
        return &self.state.devices;
    }
}
