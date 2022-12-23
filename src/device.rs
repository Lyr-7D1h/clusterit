use parser::Module;
use serde::{Deserialize, Serialize};

use log::error;

use crate::{
    connection::Authentication,
    error::ClusteritError,
    executer::{Executer, ExecuterState},
    Connection, Destination,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub destination: Destination,
    pub authentication: Option<Authentication>,
    pub executer_state: ExecuterState,
}

impl Device {
    pub fn new(
        destination: Destination,
        authentication: Option<Authentication>,
        executer_state: ExecuterState,
    ) -> Device {
        Device {
            destination,
            authentication,
            executer_state,
        }
    }

    /// Authenticate against `self.destination` and run `module` on it using an `Executer`
    pub fn run(&mut self, module: Module) -> Result<(), ClusteritError> {
        let connection = match &self.authentication {
            Some(authentication) => match Connection::connect(&self.destination, authentication) {
                Ok(c) => c,
                Err(e) => {
                    error!("Could not log into device with saved credentials: {e}");
                    let (connection, authentication) =
                        Connection::connect_interactive(&self.destination)?;
                    self.authentication = Some(authentication);
                    connection
                }
            },
            None => {
                let (connection, authentication) =
                    Connection::connect_interactive(&self.destination)?;
                self.authentication = Some(authentication);
                connection
            }
        };

        Executer::new(connection).run(&mut self.executer_state, module);

        return Ok(())
    }
}
