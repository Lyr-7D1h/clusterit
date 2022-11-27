use log::warn;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::{
    connection::Authentication,
    error::{ClusteritError, ClusteritErrorKind},
    executer::ExecuterState,
    Destination,
};

#[derive(Deserialize, Debug)]
pub struct Device {
    pub destination: Destination,
    pub authentication: Authentication,
    pub executor_state: ExecuterState,
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub devices: Vec<Device>,
}

impl State {
    pub fn from_file(path: &PathBuf) -> Result<State, ClusteritError> {
        if !path.is_file() {
            warn!("State file not found, loading empty state file");
            return Ok(State::default());
        }

        let content = fs::read_to_string(path).or(Err(ClusteritError::new(
            ClusteritErrorKind::ParseError,
            format!("Could not read state from: {path:?}"),
        )))?;

        let state: State = serde_json::from_str(&content)?;

        return Ok(state);
    }

    pub fn add_device(
        &mut self,
        destination: Destination,
        authentication: Authentication,
        executor_state: ExecuterState,
    ) -> &Device {
        let device = Device {
            destination,
            authentication,
            executor_state,
        };
        self.devices.push(device);
        return &device;
    }

    pub fn get_device(&self, destination: &Destination) -> Option<&Device> {
        self.devices
            .iter()
            .find(|d| d.destination.hostname == destination.hostname)
    }
}

impl Default for State {
    fn default() -> Self {
        Self { devices: vec![] }
    }
}
