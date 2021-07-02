use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf};

pub mod setup;

mod connection;

mod step_executer;

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
    initial_server: bool,
    architecture: Architecture,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Setup {
    step: u8,
    ip: IpAddr,
}

pub struct K3main {
    state: state::State,
}

impl K3main {
    pub fn init(ssh_pub_key_path: &PathBuf) -> anyhow::Result<()> {
        state::State::init(&ssh_pub_key_path)?;

        Ok(())
    }
    pub fn load() -> anyhow::Result<K3main> {
        let state = state::State::load()?;

        Ok(K3main { state })
    }
}
