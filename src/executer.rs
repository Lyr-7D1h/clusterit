// use parser::Module;

use parser::Module;
use serde::{Deserialize, Serialize};

use super::connection::Connection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecuterState {
    step: u32,
}

impl Default for ExecuterState {
    fn default() -> Self {
        ExecuterState { step: 0 }
    }
}

pub struct Executer {
    connection: Connection,
}

impl Executer {
    pub fn new(connection: Connection) -> Executer {
        Executer { connection }
    }

    pub fn run(&mut self, state: &mut ExecuterState, module: Module) {
        for step in module.steps.iter() {
            for command in step.commands.iter() {
                self.connection.exec(&command.command);
            }
        }
    }
}
