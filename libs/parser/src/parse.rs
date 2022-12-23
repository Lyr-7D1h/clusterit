use std::{
    collections::HashMap,
    env::current_dir,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    error::Error,
    parse_arena::{NodeId, ParseArena},
};

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
pub enum NodeType {
    Module,
    Step,
    Comment,
    Command,
    Copy,
    FallbackCommand,
    Argument,
}

#[derive(Debug, PartialEq)]
pub struct NodeData {
    pub node_type: NodeType,
    pub lineno: u32,
    pub value: Option<String>,
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

/// Parse a single clusterfile line and add corresponding nodes
fn parse_line(
    cwd: &Path,
    arena: &mut ParseArena<NodeData>,
    parent: NodeId,
    lineno: u32,
    line: impl Into<String>,
) -> Result<(), Error> {
    let line: &String = &line.into();

    let n = match get_value(line, 0, 3).as_str() {
        "cmd" => arena.add_node(NodeData::new_with_value(
            NodeType::Command,
            lineno,
            get_value(line, 4, line.len()),
        )),
        "fmd" => arena.add_node(NodeData::new_with_value(
            NodeType::FallbackCommand,
            lineno,
            get_value(line, 4, line.len()),
        )),
        "cpy" => arena.add_node(NodeData::new_with_value(
            NodeType::Copy,
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
            let path = cwd.join(filename);
            let mod_arena = parse(&path)?;
            let (start, _) = arena.append(mod_arena);
            start
        }
        v => {
            if line.starts_with("#") {
                arena.add_node(NodeData::new_with_value(
                    NodeType::Comment,
                    lineno,
                    get_value(line, 1, line.len()),
                ))
            } else {
                return Err(Error::parse_error(
                    lineno,
                    format!("Invalid expression: {v}"),
                ));
            }
        }
    };

    arena.make_parent(parent, n);

    return Ok(());
}

/// Parse the structure of a cluterfile to a list of nodes
pub fn parse_from_reader(
    mut reader: impl BufRead,
    cwd: &Path,
    module_name: String,
) -> Result<ParseArena<NodeData>, Error> {
    let mut arena = ParseArena::new();

    let root = arena.add_node(NodeData::new_module(0, module_name.clone()));
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
                if arena.get_node(&id).data.node_type != NodeType::Step {
                    let step = arena.add_node(NodeData::new_step(lineno));
                    arena.make_parent(root, step);
                    parent = step;
                }
            }

            line.clear();
            read = reader.read_line(line)?;
            continue;
        }

        parse_line(&cwd, &mut arena, parent, lineno, line.trim())?;

        line.clear();
        read = reader.read_line(line)?;
    }

    if arena.count() <= 2 {
        return Err(Error::parse_error(
            0,
            format!("Module {module_name} must contain at least one expression."),
        ));
    }

    // If last node is a step remove
    if let Some(id) = arena.last() {
        if arena.get_node(&id).data.node_type == NodeType::Step {
            arena.remove_node(&id);
        }
    }

    return Ok(arena);
}

/// Parse the structure of a cluterfile to a list of nodes
pub fn parse(path: &Path) -> Result<ParseArena<NodeData>, Error> {
    let cwd = path.parent().unwrap_or(Path::new("/"));
    let filename = match path.file_name() {
        Some(f) => f,
        None => {
            return Err(Error::io_error(
                std::io::ErrorKind::InvalidInput,
                format!("Could not parse filename from {path:?}"),
            ));
        }
    };

    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    return parse_from_reader(reader, cwd, filename.to_string_lossy().to_string());
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
cmd mkdir asdf
fmd rm -r asdf


# Second step
cmd echo 'second step'
";

    let arena = parse_from_reader(
        test_clusterfile.as_bytes(),
        Path::new("/tmp"),
        "test".into(),
    )
    .unwrap();

    let root = arena.root().unwrap();

    assert_eq!(root.data, NodeData::new_module(0, "test".into()));
    let mut step = arena.get_node(&root.children[0]);
    assert_eq!(
        arena.get_node(&step.children[0]).data,
        NodeData::new_with_value(NodeType::Command, 1, "echo $testfile".into())
    );
    assert_eq!(
        arena.get_node(&step.children[1]).data,
        NodeData::new_with_value(NodeType::Command, 2, "ls -al".into())
    );
    assert_eq!(
        arena.get_node(&step.children[2]).data,
        NodeData::new_with_value(NodeType::Command, 3, "mkdir asdf".into())
    );
    assert_eq!(
        arena.get_node(&step.children[3]).data,
        NodeData::new_with_value(NodeType::FallbackCommand, 4, "rm -r asdf".into())
    );

    step = arena.get_node(&root.children[1]);
    assert_eq!(
        arena.get_node(&step.children[0]).data,
        NodeData::new_with_value(NodeType::Comment, 7, " Second step".into())
    );
    assert_eq!(
        arena.get_node(&step.children[1]).data,
        NodeData::new_with_value(NodeType::Command, 8, "echo 'second step'".into())
    );
}
