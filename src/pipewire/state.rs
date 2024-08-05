use std::collections::HashMap;

use tracing::{debug, warn};

use super::node::Node;

#[derive(Debug)]
pub struct State {
    nodes: HashMap<u32, Node>,
}

impl State {
    pub fn new() -> Self {
        State {
            nodes: HashMap::new(),
        }
    }

    pub fn change_node(&mut self, node: Node) -> &mut Self {
        debug!("change node {:#?}", node);
        let store_node = self.nodes.get_mut(&node.id);
        if let Some(store_node) = store_node {
            *store_node = node
        } else {
            self.nodes.insert(node.id, node);
        }
        self
    }

    pub fn get_node(&mut self, node_id: u32) -> &mut Node {
        let state = self.nodes.get_mut(&node_id).unwrap();
        debug!("get node {:#?}", state);
        state
    }

    pub fn remove_node(&mut self, node_id: u32) {
        let node = self.nodes.remove(&node_id);
        debug!("remove node {:?}", node);
    }
}
