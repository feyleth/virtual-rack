use std::collections::HashMap;

use tokio::sync::broadcast;

use super::node::{Node, NodeChangeEvent, NodeValue};

#[derive(Clone, Debug)]
pub struct NodeChange {
    pub id: u32,
    pub event: NodeChangeEvent,
}
#[derive(Clone, Debug)]
pub enum StateChangeEvent {
    AddNode(Node),
}

#[derive(Debug)]
pub struct State {
    nodes: HashMap<u32, Node>,
    broadcast: broadcast::Sender<StateChangeEvent>,
}

impl State {
    pub fn new(broadcast: broadcast::Sender<StateChangeEvent>) -> Self {
        State {
            nodes: HashMap::new(),
            broadcast,
        }
    }

    pub fn change_node(&mut self, node: NodeValue) -> &mut Self {
        let store_node = self.nodes.get_mut(&node.id);
        if let Some(store_node) = store_node {
            store_node.apply_diff(node);
        } else {
            let id = node.id;
            self.nodes.insert(id, Node::new(node));
            let node = self.nodes.get(&id).expect("node exist");

            let _ = self.broadcast.send(StateChangeEvent::AddNode(node.clone()));
        }
        self
    }

    pub fn get_node(&mut self, node_id: u32) -> &mut Node {
        let state = self.nodes.get_mut(&node_id).unwrap();
        state
    }

    pub fn remove_node(&mut self, node_id: u32) {
        let node = self.nodes.remove_entry(&node_id);
        if let Some((_, node)) = node {
            node.remove();
        }
    }

    pub fn subscribe_event(&self) -> broadcast::Receiver<StateChangeEvent> {
        self.broadcast.subscribe()
    }
}
