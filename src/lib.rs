use std::path::Path;
use std::path::PathBuf;

pub mod state;
use executer::ExecuterState;
use log::debug;
use parser::Module;
use state::State;

mod connection;
pub use connection::Connection;
pub use connection::Destination;

mod device;
use device::Device;

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
        &mut self,
        module_path: &Path,
        destination: Destination,
        arguments: Vec<String>,
    ) -> Result<(), ClusteritError> {
        debug!("Parsing module: {module_path:?}");
        let module = Module::new(module_path, arguments)?;

        match self.state.get_device(&destination) {
            Some(device) => {
                debug!("Found device");
                device.run(module)?;
            }
            None => {
                debug!("No existing device found");
                let mut device = Device::new(destination, None, ExecuterState::default());
                device.run(module)?;
                self.state.add_device(device);
            }
        };

        return Ok(());
    }

    pub fn devices(&self) -> &Vec<Device> {
        return self.state.devices();
    }
}
