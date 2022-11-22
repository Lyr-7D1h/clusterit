use std::path::PathBuf;

use crate::{error::Error, parse::parse, parse_arena::ParseArena};

pub struct FallbackCommand {
    pub command: String,
}

pub struct Command {
    pub command: String,
    pub fallback: Option<FallbackCommand>,
}

pub enum Expression {
    Module(Module),
    Command(Command),
}
pub struct Step {
    pub commands: Vec<Command>,
}

/// Code abstraction for a module
pub struct Module {
    pub name: String,
    pub steps: Vec<Step>,
}

impl Module {
    /// Will parse and validate everything that is in the ParseArena
    pub fn new(file_path: PathBuf, arguments: Vec<String>) -> Result<Module, Error> {
        let arena = parse(file_path)?;

        let root = arena.root().expect("no root found");
        let name = root
            .data
            .value
            .clone()
            .expect("module does not have a name");
        for id in root.children.iter() {
            let child = arena.get_node(id);
        }

        println!("{arena}");
        let steps: Vec<Step> = vec![];

        Ok(Module { steps, name })
    }
}
