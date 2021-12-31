use log::{debug, error};

use super::connection::Connection;

pub struct StepExecuter {
    steps: Vec<String>,
    connection: Connection,
    current_step: usize,
}

impl StepExecuter {
    pub fn new(connection: Connection) -> StepExecuter {
        StepExecuter {
            steps: vec![],
            connection,
            current_step: 0,
        }
    }

    pub fn set_step(&mut self, step: usize) {
        self.current_step = step
    }

    pub fn add_step(&mut self, command: &str) {
        self.steps.push(command.to_owned())
    }

    pub fn exec(&self) -> Result<(), usize> {
        for (step, command) in self.steps.iter().skip(self.current_step).enumerate() {
            let step = step + 1;

            let result = match self.connection.exec(command) {
                Ok(result) => result,
                Err(e) => {
                    error!("Failed to exec `{}`: {}", command, e);
                    return Err(step);
                }
            };

            debug!("{}", result.stdout);
            if result.exit_code > 0 {
                error!("{}", result.stderr);
                error!(
                    "Step `{}` failed with exit code `{}`",
                    step, result.exit_code
                );
                return Err(step);
            }
        }

        Ok(())
    }
}
