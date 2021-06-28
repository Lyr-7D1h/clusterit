use std::error::Error;

mod config;
pub mod setup;

mod state;

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
