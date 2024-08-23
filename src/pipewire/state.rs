use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast;

use super::node::{Link, LinkValue, Node, NodeValue};

#[derive(Clone, Debug)]
pub enum StateChangeEvent {
    AddNode(Node),
    AddLink(Link),
}

#[derive(Clone, Debug)]
pub struct StateValue {
    pub nodes: HashMap<u32, Node>,
    pub links: HashMap<u32, Link>,
}

#[derive(Clone)]
pub struct State {
    value: Arc<Mutex<StateValue>>,
    port_map: Arc<Mutex<HashMap<u32, u32>>>,
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
                links: HashMap::new(),
            })),
            port_map: Arc::new(Mutex::new(HashMap::new())),
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

    pub fn get_node(&self, node_id: u32) -> Option<Node> {
        self.value
            .lock()
            .expect("Faile to get mutex")
            .nodes
            .get(&node_id)
            .map(|node| node.clone())
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
    pub(crate) fn change_link(&self, link: LinkValue) -> &Self {
        let mut state = self.value.lock().expect("Faile to get mutex");
        if let Some(store_node) = state.links.get_mut(&link.id) {
            store_node.apply_diff(link);
        } else {
            let id = link.id;
            state.links.insert(id, Link::new(link));
            let node = state.links.get(&id).expect("node exist");

            let _ = self.broadcast.send(StateChangeEvent::AddLink(node.clone()));
        }
        self
    }

    pub fn get_link(&self, link_id: u32) -> Link {
        self.value
            .lock()
            .expect("Faile to get mutex")
            .links
            .get_mut(&link_id)
            .unwrap()
            .clone()
    }

    pub(crate) fn remove_link(&self, link_id: u32) {
        let node = self
            .value
            .lock()
            .expect("Faile to get mutex")
            .links
            .remove_entry(&link_id);
        if let Some((_, node)) = node {
            node.remove();
        }
    }
    pub(crate) fn add_map_port(&self, port_id: u32, node_id: u32) {
        self.port_map
            .lock()
            .expect("Faile to get mutex")
            .insert(port_id, node_id);
    }
    pub(crate) fn modify_map_port(&self, port_id: u32, node_id: u32) {
        self.port_map
            .lock()
            .expect("Faile to get mutex")
            .entry(port_id)
            .and_modify(|value| *value = node_id);
    }
    pub(crate) fn get_map_port(&self, port_id: u32) -> Option<u32> {
        self.port_map
            .lock()
            .expect("Faile to get mutex")
            .get(&port_id)
            .map(|value| value.clone())
    }

    pub fn subscribe(&self) -> (StateValue, broadcast::Receiver<StateChangeEvent>) {
        let state = self.value.lock().expect("Faile to get mutex");
        let subscribe = self.broadcast.subscribe();
        ((*state).clone(), subscribe)
    }

    pub fn get(&self) -> StateValue {
        let state = self.value.lock().expect("Faile to get mutex");
        state.clone()
    }
}
