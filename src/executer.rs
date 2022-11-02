use parser::Module;

use super::connection::Connection;

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
    state: ExecuterState,
}

impl Executer {
    pub fn from_state(connection: Connection, state: ExecuterState) -> Executer {
        Executer { connection, state }
    }

    pub fn new(connection: Connection) -> Executer {
        let state = ExecuterState::default();
        Executer { connection, state }
    }

    pub fn run(&mut self, module: Module) {
        for step in module.steps.iter() {
            for command in step.commands.iter() {
                self.connection.exec(&command.command);
            }
        }
    }
}
