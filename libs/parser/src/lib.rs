use std::io::{self, BufRead, ErrorKind};

use crate::parse_arena::{NodeType, ParseArena};

mod error;
mod parse_arena;

pub struct FallbackCommand {}

pub struct Command {
    pub command: String,
    pub fallback: Option<FallbackCommand>,
}

pub enum Execution {
    Command(Command),
    Argument,
    Module,
}

pub struct Step {
    pub commands: Vec<Command>,
}

pub struct Module {
    pub steps: Vec<Step>,
}

// https://github.com/moby/buildkit/blob/master/frontend/dockerfile/parser/parser.go
/// Parse to an code abstraction of the clusterfile
pub fn parse(reader: impl BufRead) -> Result<Module, io::Error> {
    let mut parse_tree = ParseArena::new();
    parse_tree.parse_from_reader(reader)?;

    let mut steps = vec![];
    let mut commands = vec![];

    for node in &mut parse_tree.nodes {
        match node.node_type {
            NodeType::Module => (),
            NodeType::Step => commands.clear(),
            NodeType::Expression => match node.value {
                Some() => todo!(),
                None => {
                    return io::Error::new(
                        ErrorKind::InvalidData,
                        format!("expression must have an value"),
                    )
                }
            },
        }
        steps.push(Step { commands });
    }
    println!("{parse_tree:?}");

    return Ok(Module { steps });
}

#[test]
fn test_parse() {
    let test_clusterfile = "CMD echo $testfile
CMD ls -al

CMD echo 'second step'";

    parse(test_clusterfile.as_bytes());
}
