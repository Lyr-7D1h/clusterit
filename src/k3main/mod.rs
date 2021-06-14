use std::error::Error;

mod config;
pub mod setup;

pub struct K3main {
    _config: config::Config,
}

impl K3main {
    pub fn new() -> Result<K3main, Box<dyn Error>> {
        let config = config::from_file()?;

        Ok(K3main { _config: config })
    }
}
