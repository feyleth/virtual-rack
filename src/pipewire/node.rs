use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeChangeEvent {
    Id(u32),
    Name(String),
    State(State),
    Remove,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeValue {
    pub id: u32,
    pub name: String,
    pub state: State,
    pub in_ports: Vec<Port>,
    pub out_ports: Vec<Port>,
}

#[derive(Clone)]
pub struct Node {
    value: Arc<Mutex<NodeValue>>,
    broadcast: broadcast::Sender<NodeChangeEvent>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value.lock().expect("Faile to get mutex");
        f.write_fmt(format_args!("{:#?}", value))
    }
}

impl Node {
    pub(crate) fn new(node_value: NodeValue) -> Self {
        let (broadcast, _) = broadcast::channel(25);
        Node {
            value: Arc::new(Mutex::new(node_value)),
            broadcast,
        }
    }

    pub(crate) fn apply_diff(&self, node: NodeValue) {
        {
            let mut node_value = self.value.lock().expect("Faile to get mutex");
            if node_value.id != node.id {
                node_value.id = node.id;
                let _ = self.broadcast.send(NodeChangeEvent::Id(node.id));
            }
        }
        {
            let mut node_value = self.value.lock().expect("Faile to get mutex");
            if node_value.name != node.name {
                node_value.name = node.name.clone();
                let _ = self.broadcast.send(NodeChangeEvent::Name(node.name));
            }
        }
        self.change_state(node.state);
    }

    pub(crate) fn change_state(&self, state: State) -> &Self {
        let mut node_value = self.value.lock().expect("Faile to get mutex");
        if node_value.state != state {
            node_value.state = state.clone();
            let _ = self.broadcast.send(NodeChangeEvent::State(state));
        }
        self
    }

    pub fn value(&self) -> NodeValue {
        self.value.lock().expect("Faile to get mutex").clone()
    }

    pub fn subcribe(&self) -> (NodeValue, broadcast::Receiver<NodeChangeEvent>) {
        let node = self.value.lock().expect("Faile to get mutex");
        let subscribe = self.broadcast.subscribe();
        ((*node).clone(), subscribe)
    }
    pub(crate) fn remove(&self) {
        let _ = self.broadcast.send(NodeChangeEvent::Remove);
    }
}

#[cfg(test)]
mod test {
    use crate::pipewire::node::{NodeChangeEvent, NodeValue};

    use super::Node;

    #[tokio::test]
    async fn subcribe_after_change() {
        let node = Node::new(super::NodeValue {
            id: 1,
            name: "test".to_owned(),
            state: super::State::Idle,
            in_ports: vec![],
            out_ports: vec![],
        });
        let (new_node, mut events) = node.subcribe();
        node.change_state(super::State::Running);

        assert_eq!(
            new_node,
            NodeValue {
                id: 1,
                name: "test".to_owned(),
                state: crate::pipewire::node::State::Idle,
                in_ports: vec![],
                out_ports: vec![]
            }
        );
        assert_eq!(
            events.recv().await,
            Ok(NodeChangeEvent::State(
                crate::pipewire::node::State::Running
            ))
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Creating,
    Suspended,
    Idle,
    Running,
    Error(String),
}

impl From<pipewire::node::NodeState<'_>> for State {
    fn from(value: pipewire::node::NodeState) -> Self {
        match value {
            pipewire::node::NodeState::Error(e) => Self::Error(e.to_string()),
            pipewire::node::NodeState::Creating => Self::Creating,
            pipewire::node::NodeState::Suspended => Self::Suspended,
            pipewire::node::NodeState::Idle => Self::Idle,
            pipewire::node::NodeState::Running => Self::Running,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    pub id: u32,
}
