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

use std::io::BufRead;

#[derive(Debug)]
enum NodeType {
    Module,
    Step,
    Expression,
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    line: u32,
    value: Option<String>,
    children: Vec<Node>,
}

impl Node {
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

fn get_value(line: &String, start: usize, end: usize) -> String {
    let bytes = line.chars().skip(start);
    return bytes.take(end - start).collect();
}

#[derive(Debug)]
struct ParseTree {
    root: Node,
}

impl ParseTree {
    fn parse(line: &mut String, mut reader: impl BufRead, parent: &mut Node) {
        if reader.read_line(line).is_err() {
            return;
        }

        let n = match &line[0..3] {
            "CMD" => Node {
                node_type: NodeType::Module,
                line: 0,
                value: Some(get_value(line, 4, line.len())),
                children: vec![],
            },
            _ => panic!("Invalid execution"),
        };

        parent.add_child(n);

        line.clear();
    }

    pub fn from_reader(mut reader: impl BufRead) -> ParseTree {
        let mut line = &mut String::new();

        let mut root = Node {
            node_type: NodeType::Module,
            line: 0,
            value: None,
            children: vec![],
        };

        let mut step = Node {
            node_type: NodeType::Step,
            line: 0,
            value: None,
            children: vec![],
        };

        ParseTree::parse(&mut line, reader, &mut step);

        root.add_child(step);

        return ParseTree { root };
    }
}

// https://github.com/moby/buildkit/blob/master/frontend/dockerfile/parser/parser.go
pub fn parse(reader: impl BufRead) {
    let parse_tree = ParseTree::from_reader(reader);

    println!("{parse_tree:?}");

    todo!()
}
