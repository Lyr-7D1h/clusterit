use std::{
    collections::HashMap,
    env::current_dir,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use error::Error;
use parse_arena::{NodeId, ParseArena};

mod error;
mod parse_arena;

/// Get a value safely from start to finish
fn get_value<T: Into<String>>(value: T, start: usize, end: usize) -> String {
    let value: Vec<char> = value.into().chars().collect();
    let len = value.len();
    let mut v: Vec<char> = vec![];
    for i in start..end {
        if i < len {
            v.push(value[i])
        }
    }

    return v.into_iter().collect();
}

/// Specify what kind of value the node has
#[derive(Debug, PartialEq)]
enum NodeType {
    Module,
    Step,
    Comment,
    Command,
    FallbackCommand,
    Argument,
}

#[derive(Debug)]
struct NodeData {
    node_type: NodeType,
    lineno: u32,
    value: Option<String>,
}

impl NodeData {
    pub fn new_step(lineno: u32) -> NodeData {
        NodeData {
            node_type: NodeType::Step,
            lineno,
            value: None,
        }
    }

    pub fn new_module(lineno: u32, value: String) -> NodeData {
        NodeData {
            node_type: NodeType::Module,
            lineno,
            value: Some(value),
        }
    }

    pub fn new_with_value(node_type: NodeType, lineno: u32, value: String) -> NodeData {
        NodeData {
            node_type,
            lineno,
            value: Some(value),
        }
    }
}

impl Display for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {}",
            self.node_type,
            self.value.clone().map_or("".into(), |x| format!("({x})"))
        )
    }
}

struct Parser {
    variables: HashMap<String, String>,
    cwd: PathBuf,
}

impl Parser {
    pub fn new(cwd: impl Into<PathBuf>) -> Parser {
        Parser {
            variables: HashMap::new(),
            cwd: cwd.into(),
        }
    }

    /// Parse a single clusterfile line and add corresponding nodes
    fn parse_line(
        &mut self,
        arena: &mut ParseArena<NodeData>,
        parent: NodeId,
        lineno: u32,
        line: impl Into<String>,
    ) -> Result<(), Error> {
        let line: &String = &line.into();

        if line.starts_with("#") {
            arena.add_node(NodeData::new_with_value(
                NodeType::Comment,
                lineno,
                get_value(line, 1, line.len()),
            ));

            return Ok(());
        }

        let n = match get_value(line, 0, 3).as_str() {
            "cmd" => arena.add_node(NodeData::new_with_value(
                NodeType::Command,
                lineno,
                get_value(line, 4, line.len()),
            )),
            "fmd" => arena.add_node(NodeData::new_with_value(
                NodeType::Command,
                lineno,
                get_value(line, 4, line.len()),
            )),
            "arg" => arena.add_node(NodeData::new_with_value(
                NodeType::Argument,
                lineno,
                get_value(line, 4, line.len()),
            )),
            "mod" => {
                let filename = get_value(line, 4, line.len());
                let mod_arena = self.parse(filename)?;
                let (start, _) = arena.append(mod_arena);
                start
            }
            v => {
                return Err(Error::parse_error(
                    lineno,
                    format!("Invalid expression: {v}"),
                ))
            }
        };

        arena.make_parent(parent, n);

        return Ok(());
    }

    /// Parse the structure of a cluterfile to a list of nodes
    pub fn parse_from_reader(
        &mut self,
        mut reader: impl BufRead,
        module_name: String,
    ) -> Result<ParseArena<NodeData>, Error> {
        let mut arena = ParseArena::new();
        let root = arena.add_node(NodeData::new_module(0, module_name));
        let step = arena.add_node(NodeData::new_step(0));
        arena.make_parent(root, step);

        let line = &mut String::new();
        let mut lineno = 0;
        let mut parent = step;
        let mut read = reader.read_line(line)?;
        while read != 0 {
            lineno += 1;

            if line == "\n" {
                if let Some(id) = arena.last() {
                    if arena.get_node(&id).value.node_type != NodeType::Step {
                        let step = arena.add_node(NodeData::new_step(lineno));
                        arena.make_parent(root, step);
                        parent = step;
                    }
                }

                line.clear();
                read = reader.read_line(line)?;
                continue;
            }

            self.parse_line(&mut arena, parent, lineno, line.trim())?;

            line.clear();
            read = reader.read_line(line)?;
        }

        // If last node is a step remove
        if let Some(id) = arena.last() {
            if arena.get_node(&id).value.node_type == NodeType::Step {
                arena.remove_node(&id);
            }
        }

        return Ok(arena);
    }

    /// Parse the structure of a cluterfile to a list of nodes
    fn parse(&mut self, filename: String) -> Result<ParseArena<NodeData>, Error> {
        let mut path = self.cwd.clone();
        path.push(&filename);
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        return self.parse_from_reader(reader, filename);
    }
}

#[test]
fn test_get_value() {
    let mut value = get_value("THIS IS AN TEST", 0, 3);
    assert_eq!(value, "THI");

    value = get_value("TH", 0, 3);
    assert_eq!(value, "TH");
}

#[test]
fn test_parse() {
    let test_clusterfile = "cmd echo $testfile
cmd ls -al


cmd echo 'second step'
";
    let mut parser = Parser::new(current_dir().unwrap());

    let arena = parser
        .parse_from_reader(test_clusterfile.as_bytes(), "test".into())
        .unwrap();

    println!("{arena}");
    panic!("a");
}
