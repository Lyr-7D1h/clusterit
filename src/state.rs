use log::{warn, debug};
use path_resolver::resolve_path;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    device::Device,
    error::{ClusteritError, ClusteritErrorKind},
    Destination,
};

#[derive(Serialize, Deserialize, Debug)]
struct InnerState {
    devices: Vec<Device>,
}

impl Default for InnerState {
    fn default() -> Self {
        Self { devices: vec![] }
    }
}

#[derive(Debug)]
pub struct State {
    inner_state: InnerState,
    path: PathBuf,
}

impl State {
    pub fn from_file(path: &Path) -> Result<State, ClusteritError> {
        if !path.is_file() {
            warn!("State file not found, loading empty state file");
            return Ok(State {
                inner_state: InnerState::default(),
                path: resolve_path(path),
            });
        }

        let content = fs::read_to_string(path).or(Err(ClusteritError::new(
            ClusteritErrorKind::ParseError,
            format!("Could not read state from: {path:?}"),
        )))?;

        let inner_state: InnerState = serde_json::from_str(&content)?;

        return Ok(State {
            inner_state,
            path: path.to_path_buf(),
        });
    }

    pub fn save(&self) -> Result<(), ClusteritError> {
        fs::write(&self.path, serde_json::to_string(&self.inner_state)?)?;

        return Ok(());
    }

    pub fn add_device(&mut self, device: Device) {
        debug!(
            "Adding device tot state with destination: {}",
            device.destination
        );
        self.inner_state.devices.push(device);
    }

    pub fn devices(&self) -> &Vec<Device> {
        &self.inner_state.devices
    }

    pub fn get_device(&mut self, destination: &Destination) -> Option<&mut Device> {
        self.inner_state
            .devices
            .iter_mut()
            .find(|d| d.destination.hostname == destination.hostname)
    }
}
