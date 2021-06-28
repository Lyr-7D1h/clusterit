use std::{
    fs::{read_to_string, File},
    io::Write,
    net::IpAddr,
    path::PathBuf,
    str::FromStr,
};

use anyhow::Context;
use dirs::home_dir;
use log::info;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

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
pub struct State {
    node_token_secret: String,
    node_token: Option<String>,
    ssh_key_path: PathBuf,
    ip_offset: u32,
    ip_gateway: IpAddr,
    servers: Vec<Server>,
}

fn get_ssh_key_path() -> anyhow::Result<PathBuf> {
    let ssh_dir = home_dir()
        .ok_or(anyhow::Error::msg("Could not find home"))?
        .join(".ssh");

    for entry in ssh_dir.read_dir()? {
        let entry = entry?;

        if entry.path().is_file() {
            let filename = entry
                .file_name()
                .into_string()
                .expect("Could not get filename");

            println!("{}", filename);
            match filename.as_str() {
                "id_ed25519.pub" => return Ok(entry.path()),
                "id_rsa.pub" => return Ok(entry.path()),
                "id_ecdsa.pub" => return Ok(entry.path()),
                _ => {}
            };
        }
    }

    return Err(anyhow::Error::msg("Could not find a SSH key"));
}

fn get_default_state() -> State {
    State {
        node_token: None,
        node_token_secret: thread_rng()
            .sample_iter(&Alphanumeric)
            .take(40)
            .map(char::from)
            .collect(),
        ssh_key_path: get_ssh_key_path().unwrap(),
        ip_offset: 100,
        ip_gateway: "192.168.2.254".parse().unwrap(),
        servers: vec![],
    }
}

impl State {
    pub fn new() -> anyhow::Result<State> {
        let path = PathBuf::from_str("./state.json")?;

        if !path.exists() {
            let default_state = get_default_state();
            let default_state_json = serde_json::to_string(&default_state)?;

            info!("Writing default state to file {:?}", path);
            let mut file = File::create(&path)?;
            file.write_all(default_state_json.as_bytes())?;
        }

        info!("Reading {:?}", path);
        let content = read_to_string(path).with_context(|| "Could not read state.json")?;
        let state = serde_json::from_str(&content)?;

        Ok(state)
    }
}
