use std::{fs, path::PathBuf};
use serde::Deserialize;

use super::error::ClusteritError;

#[derive(Deserialize, Debug)]
pub struct Device {
    pub destination: String,
    pub ssh_port: u16,
    pub ssh_key: String
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub devices: Vec<Device>
}

impl State {
    pub fn from_file(path: &PathBuf) -> Result<State, ClusteritError> {
        let content = fs::read_to_string(path).or(Err(ClusteritError::ParseError(
            format!("Could not read config from: {path:?}"),
        )))?;

        let state: State = serde_json::from_str(&content)?;

        return Ok(state)
    }
}
