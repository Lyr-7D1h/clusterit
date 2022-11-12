use std::io::{self, BufRead};

#[derive(Debug)]
pub enum NodeType {
    Module,
    Step,
    Expression,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    index: usize,
}

#[derive(Debug)]
pub struct Node {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,

    pub node_type: NodeType,
    pub value: Option<String>,
    pub line: u32,
}

impl Node {
    fn new(node_type: NodeType, value: Option<String>, line: u32) -> Node {
        Node {
            parent: None,
            children: vec![],

            node_type,
            value,
            line,
        }
    }

    fn add_child(&mut self, id: NodeId) {
        self.children.push(id)
    }
}

/// Parse Arena using region based memory (https://en.wikipedia.org/wiki/Region-based_memory_management)
// https://github.com/saschagrunert/indextree
#[derive(Debug)]
pub struct ParseArena {
    pub nodes: Vec<Node>,
}

impl ParseArena {
    pub fn new() -> ParseArena {
        ParseArena { nodes: vec![] }
    }

    pub fn add_node(&mut self, node_type: NodeType, value: Option<String>, line: u32) -> NodeId {
        let node = Node {
            parent: None,
            children: vec![],

            node_type,
            value,
            line,
        };
        self.nodes.push(node);

        NodeId {
            index: self.nodes.len() - 1,
        }
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.index]
    }

    fn make_parent(&mut self, parent_id: NodeId, child_id: NodeId) {
        let parent = self.get_node_mut(parent_id);
        parent.children.push(child_id);
        let child = self.get_node_mut(child_id);
        child.parent = Some(parent_id);
    }

    /// Recursivly parse the structure of a cluteritfile
    fn parse(
        &mut self,
        line: &mut String,
        lineno: u32,
        mut reader: impl BufRead,
        mut parent: NodeId,
    ) -> Result<(), io::Error> {
        let read = reader.read_line(line)?;

        // end of reader
        if read == 0 {
            return Ok(());
        }

        // new line means next step
        if line.len() == 0 || line == "\n" {
            parent = self.add_node(NodeType::Step, None, lineno);
            return self.parse(line, lineno + 1, reader, parent);
        }

        let trimmed_line = line.trim();

        let n = self.add_node(NodeType::Expression, Some(line), lineno);

        line.clear();

        self.make_parent(parent, n);
        self.parse(line, lineno + 1, reader, parent)?;

        return Ok(());
    }

    /// Parse the structure of a cluterfile to corresponding nodes
    pub fn parse_from_reader(&mut self, reader: impl BufRead) -> Result<(), io::Error> {
        let mut line = &mut String::new();

        let root = self.add_node(NodeType::Module, None, 0);
        let step = self.add_node(NodeType::Step, None, 0);
        self.make_parent(root, step);

        self.parse(&mut line, 1, reader, step)?;

        return Ok(());
    }
}
