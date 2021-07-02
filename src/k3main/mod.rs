use serde::{Deserialize, Serialize};
use std::{error::Error, net::IpAddr};

mod config;
pub mod setup;

mod connection;

mod state;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    K3SServer,
    K3SAgent,
    NFS,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Architecture {
    Arm,
    Arm64,
    Amd64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    ip: IpAddr,
    role: Role,
    architecture: Architecture,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Setup {
    step: u8,
    ip: IpAddr,
}

pub struct K3main {
    _config: config::Config,
    state: state::State,
}

impl K3main {
    pub fn new() -> Result<K3main, Box<dyn Error>> {
        let config = config::from_file()?;
        let state = state::State::new()?;

        Ok(K3main {
            _config: config,
            state,
        })
    }
}
