use std::{fs, path::PathBuf};

use serde::Deserialize;
use toml::{value::Map, Value};

use super::error::ClusteritError;

const DEFAULT_CONFIG: &str = include_str!("./default_config.toml");

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub port: u16,
    // pub modules: Vec<Value>,
}

// #[derive(Deserialize, Debug)]
// pub struct Server {
// pub ip: String,
// connection: Connection,
// }

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
    pub fn from_file(path: &PathBuf) -> Result<Config, ClusteritError> {
        let raw_config: Value = {
            let content = fs::read_to_string(path).or(Err(ClusteritError::ConfigParseError(
                format!("Could not read config from: {path:?}"),
            )))?;
            toml::from_str(&content)?
        };
        let mut raw_default_config: Value = toml::from_str(DEFAULT_CONFIG)?;

        merge(&mut raw_default_config, &raw_config);

        println!("{raw_config:?}");

        Ok(raw_config.try_into().unwrap())
    }
}
