use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;
use toml::{value::Map, Value};

const DEFAULT_CONFIG: &str = include_str!("./default_config.toml");

#[derive(Deserialize, Debug)]
pub struct Config {
    pub servers: HashMap<String, Server>,
}

// #[derive(Deserialize, Debug)]
// struct Connection {
//     users: Vec<String>,
//     password: Vec<String>,
// }

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: String,
    // connection: Connection,
}

// #[derive(Deserialize, Debug)]
// struct Keys {
//     url: String,
// }

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Table(ref mut a), &Value::Table(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Table(Map::new())), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Config> {
        let mut raw_config: Value = {
            let content = fs::read_to_string(path)?;
            toml::from_str(&content).context("Could not parse config")?
        };
        let raw_default_config: Value = toml::from_str(DEFAULT_CONFIG)?;

        merge(&mut raw_config, &raw_default_config);

        Ok(raw_config.try_into().unwrap())
    }
}
