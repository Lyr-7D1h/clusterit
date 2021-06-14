use dirs::home_dir;
use log::info;
use serde::Deserialize;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    ssh_key_path: Option<PathBuf>,
}

const DEFAULT_CONFIG: &'static [u8] = include_bytes!("default_config.toml");

fn validate_config(config: &Config) -> Result<(), String> {
    if let Some(ssh_key_path) = &config.ssh_key_path {
        if ssh_key_path.exists() == false {
            return Err(format!(
                "ssh_key_path ({:?}) does not exist",
                config.ssh_key_path
            ));
        }
    }

    Ok(())
}

pub fn from_file() -> Result<Config, Box<dyn Error>> {
    let config_dir = home_dir()
        .ok_or(io::Error::new(ErrorKind::Other, "Missing home directory"))?
        .join(".k3main");

    if config_dir.exists() == false {
        info!("Creating {:?}", config_dir);
        create_dir_all(&config_dir)?;
    }

    let config_file_path = config_dir.join("config.toml");

    if config_file_path.exists() == false {
        info!("Creating {:?}", config_file_path);
        let mut file = File::create(&config_file_path)?;
        file.write_all(&DEFAULT_CONFIG)?;
    }

    info!("Reading config from {:?}", config_file_path);
    let content = read_to_string(config_file_path)?;

    let config: Config = toml::from_str(&content)?;

    validate_config(&config)?;

    return Ok(config);
}
