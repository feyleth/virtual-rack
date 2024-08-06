use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast;

use super::node::{Node, NodeValue};

#[derive(Clone, Debug)]
pub enum StateChangeEvent {
    AddNode(Node),
}

#[derive(Clone, Debug)]
pub struct StateValue {
    pub nodes: HashMap<u32, Node>,
}

#[derive(Clone)]
pub struct State {
    value: Arc<Mutex<StateValue>>,
    broadcast: broadcast::Sender<StateChangeEvent>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value.lock().expect("Faile to get mutex");
        f.write_fmt(format_args!("{:#?}", value))
    }
}

impl State {
    pub fn new() -> Self {
        let (broadcast, _) = broadcast::channel(25);
        State {
            value: Arc::new(Mutex::new(StateValue {
                nodes: HashMap::new(),
            })),
            broadcast,
        }
    }

    pub(crate) fn change_node(&self, node: NodeValue) -> &Self {
        let mut state = self.value.lock().expect("Faile to get mutex");
        if let Some(store_node) = state.nodes.get_mut(&node.id) {
            store_node.apply_diff(node);
        } else {
            let id = node.id;
            state.nodes.insert(id, Node::new(node));
            let node = state.nodes.get(&id).expect("node exist");

            let _ = self.broadcast.send(StateChangeEvent::AddNode(node.clone()));
        }
        self
    }

    pub fn get_node(&self, node_id: u32) -> Node {
        self.value
            .lock()
            .expect("Faile to get mutex")
            .nodes
            .get_mut(&node_id)
            .unwrap()
            .clone()
    }

    pub(crate) fn remove_node(&self, node_id: u32) {
        let node = self
            .value
            .lock()
            .expect("Faile to get mutex")
            .nodes
            .remove_entry(&node_id);
        if let Some((_, node)) = node {
            node.remove();
        }
    }

    pub fn subscribe(&self) -> (StateValue, broadcast::Receiver<StateChangeEvent>) {
        let state = self.value.lock().expect("Faile to get mutex");
        let subscribe = self.broadcast.subscribe();
        ((*state).clone(), subscribe)
    }
}
