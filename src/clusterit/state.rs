use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    net::IpAddr,
    path::PathBuf,
    str::FromStr,
};

use anyhow::Context;
use log::info;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use super::{Server, Setup};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    node_token_secret: String,
    node_token: Option<String>,
    ssh_pub_key: String,
    ip_offset: u32,
    ip_gateway: IpAddr,
    servers: Vec<Server>,
    setup: HashMap<IpAddr, Setup>,
}

impl State {
    pub fn init(ssh_pub_key_path: &PathBuf) -> anyhow::Result<()> {
        let path = PathBuf::from_str("./state.json")?;

        if path.exists() {
            return Err(anyhow::Error::msg(format!("{:?} already exsits", path)));
        }

        let ssh_pub_key = read_to_string(ssh_pub_key_path)
            .with_context(|| format!("Could not open ssh key: {:?}", ssh_pub_key_path))?;

        if ssh_pub_key.starts_with("ssh-") == false && ssh_pub_key.starts_with("ecdsa-") == false {
            return Err(anyhow::Error::msg("Invalid public key"));
        }

        let default_state = State {
            node_token: None,
            node_token_secret: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(40)
                .map(char::from)
                .collect(),
            ssh_pub_key,
            ip_offset: 100,
            ip_gateway: "192.168.2.254".parse().unwrap(),
            servers: vec![],
            setup: HashMap::new(),
        };

        let default_state_json = serde_json::to_string(&default_state)?;

        info!("Writing state to file {:?}", path);
        let mut file = File::create(&path)?;
        file.write_all(default_state_json.as_bytes())?;

        Ok(())
    }

    pub fn load() -> anyhow::Result<State> {
        let path = PathBuf::from_str("./state.json")?;

        info!("Reading {:?}", path);
        let content = read_to_string(path).with_context(|| "Could not read state.json")?;
        let state = serde_json::from_str(&content)?;
        Ok(state)
    }

    pub fn get_pub_ssh_key(&self) -> &String {
        &self.ssh_pub_key
    }

    // pub fn add_setup(&mut self, setup: Setup) {
    //     self.setup.push(setup)
    // }

    pub fn find_setup(&self, ip: &IpAddr) -> Option<&Setup> {
        self.setup.get(ip)
        // self.setup.into_iter().find(|s| s.ip == ip)
    }

    pub fn remove_setup(mut self, setup: Setup) {
        // self.setup = self
        //     .setup
        //     .into_iter()
        //     .filter(|s| s.ip == setup.ip)
        //     .collect()
    }
}
