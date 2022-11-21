use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    pub fn new(index: usize) -> NodeId {
        NodeId { index }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,

    pub value: T,
}

/// Parse Arena using region based memory (https://en.wikipedia.org/wiki/Region-based_memory_management)
// https://github.com/saschagrunert/indextree
#[derive(Debug)]
pub struct ParseArena<T> {
    nodes: Vec<Node<T>>,
    last: Option<NodeId>,
}

impl<T> ParseArena<T> {
    pub fn new() -> ParseArena<T> {
        ParseArena {
            nodes: vec![],
            last: None,
        }
    }

    pub fn last(&self) -> Option<NodeId> {
        let length = self.nodes.len();
        if length == 0 {
            return None;
        }

        Some(NodeId::new(length - 1))
    }

    /// Append another parse arena to the current with, returns start and end NodeIds
    pub fn append(&mut self, mut arena: ParseArena<T>) -> (NodeId, NodeId) {
        let start = self.nodes.len() - 1;
        self.nodes.append(&mut arena.nodes);
        let end = self.nodes.len() - 1;

        (NodeId::new(start), NodeId::new(end))
    }

    pub fn remove_node(&mut self, id: &NodeId) {
        self.nodes.remove(id.index);
    }

    pub fn add_node(&mut self, value: T) -> NodeId {
        let node = Node {
            parent: None,
            children: vec![],

            value,
        };
        self.nodes.push(node);

        NodeId {
            index: self.nodes.len() - 1,
        }
    }

    pub fn get_node(&self, id: &NodeId) -> &Node<T> {
        &self.nodes[id.index]
    }
    pub fn get_node_mut(&mut self, id: &NodeId) -> &mut Node<T> {
        &mut self.nodes[id.index]
    }

    pub fn make_parent(&mut self, parent_id: NodeId, child_id: NodeId) {
        let parent = self.get_node_mut(&parent_id);
        parent.children.push(child_id);
        let child = self.get_node_mut(&child_id);
        child.parent = Some(parent_id);
    }
}

impl<T: Display + Debug> Display for ParseArena<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut representation = vec![];

        if let Some(mut root) = self.nodes.first() {
            while let Some(parent) = root.parent {
                root = self.get_node(&parent);
            }

            representation.push(format!("{}", root.value));

            let level: u32 = 1;
            let leveled_children = root
                .children
                .clone()
                .into_iter()
                .map(|c| (level, c))
                .collect::<Vec<(u32, NodeId)>>();
            let mut queue = VecDeque::from(leveled_children);

            while let Some((level, child)) = queue.pop_front() {
                let child = self.get_node(&child);
                representation.push(format!("{}{}", "\t".repeat(level as usize), child.value));

                let leveled_children = child
                    .children
                    .clone()
                    .into_iter()
                    .map(|c| (level + 1, c))
                    .collect::<Vec<(u32, NodeId)>>();

                for child in leveled_children.into_iter().rev() {
                    queue.push_front(child)
                }
            }
        }

        write!(f, "{}", representation.join("\n"))
    }
}
