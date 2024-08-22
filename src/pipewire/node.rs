#[cfg(all(feature = "shuttle", test))]
use shuttle::sync::{Arc, Mutex};

use std::collections::HashMap;
#[cfg(not(all(feature = "shuttle", test)))]
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeChangeEvent {
    Name(String),
    State(NodeState),
    AddPort(Port),
    ModifyPort(Port),
    RemovePort(u32),
    Remove,
    NodeType(NodeTypeDirection),
}
#[derive(Debug, Clone, PartialEq)]
pub enum NodeTypeDirection {
    In,
    Out,
    Both,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeValue {
    pub id: u32,
    pub name: String,
    pub state: NodeState,
    pub media: Media,
    pub ports: HashMap<u32, Port>,
    pub node_type: NodeTypeDirection,
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
            if node_value.name != node.name {
                node_value.name = node.name.clone();
                let _ = self.broadcast.send(NodeChangeEvent::Name(node.name));
            }
            if node_value.node_type != node.node_type {
                node_value.node_type = node.node_type.clone();
                let _ = self
                    .broadcast
                    .send(NodeChangeEvent::NodeType(node.node_type));
            }
        }
        self.change_state(node.state);
    }

    pub(crate) fn change_state(&self, state: NodeState) -> &Self {
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

    pub fn replace_or_add_port(&self, port: Port) {
        let mut node = self.value.lock().expect("Faile to get mutex");
        let old_port = node.ports.get_mut(&port.id);
        if let Some(old_port) = old_port {
            *old_port = port.clone();
            let _ = self.broadcast.send(NodeChangeEvent::ModifyPort(port));
        } else {
            node.ports.insert(port.id, port.clone());
            let _ = self.broadcast.send(NodeChangeEvent::AddPort(port));
        }
    }
    pub fn remove_port(&self, id: u32) {
        let mut node = self.value.lock().expect("Faile to get mutex");
        let remove = node.ports.remove(&id);
        if remove.is_some() {
            let _ = self.broadcast.send(NodeChangeEvent::RemovePort(id));
        }
    }

    pub(crate) fn remove(&self) {
        let _ = self.broadcast.send(NodeChangeEvent::Remove);
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[cfg(feature = "shuttle")]
    fn subcribe_after_change() {
        use std::collections::HashMap;

        use shuttle::thread;
        use tokio::runtime::Builder;

        use crate::pipewire::node::{NodeChangeEvent, NodeValue};

        use super::{Format, Node};
        shuttle::check_random(
            move || {
                let node = Node::new(super::NodeValue {
                    id: 1,
                    name: "test".to_owned(),
                    state: super::NodeState::Idle,
                    class: Format::Audio,
                    ports: HashMap::new(),
                });

                let clone_node = node.clone();
                let subcribe_thread = thread::spawn(move || clone_node.subcribe());
                thread::spawn(move || {
                    node.change_state(super::NodeState::Running);
                })
                .join()
                .unwrap();

                let (new_node, mut events) = subcribe_thread.join().unwrap();

                if new_node.state == crate::pipewire::node::NodeState::Idle {
                    assert_eq!(
                        new_node,
                        NodeValue {
                            id: 1,
                            name: "test".to_owned(),
                            class: Format::Audio,
                            state: crate::pipewire::node::NodeState::Idle,
                            ports: HashMap::new(),
                        }
                    );

                    shuttle::future::block_on(async move {
                        assert_eq!(
                            events.recv().await,
                            Ok(NodeChangeEvent::State(
                                crate::pipewire::node::NodeState::Running
                            ))
                        );
                    });
                } else {
                    assert_eq!(
                        new_node,
                        NodeValue {
                            id: 1,
                            name: "test".to_owned(),
                            class: Format::Audio,
                            state: crate::pipewire::node::NodeState::Running,
                            ports: HashMap::new(),
                        }
                    );
                }
            },
            1000,
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeState {
    Creating,
    Suspended,
    Idle,
    Running,
    Error(String),
}

impl From<pipewire::node::NodeState<'_>> for NodeState {
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
pub enum Media {
    Audio,
    Video,
    Midi,
    Unknow(String),
    None,
}

impl From<Option<&str>> for Media {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(value) => match value {
                v if v.contains("Audio") => Media::Audio,
                v if v.contains("Midi") => Media::Midi,
                v if v.contains("Video") => Media::Video,
                value => Media::Unknow(value.to_string()),
            },
            None => Media::None,
        }
    }
}

impl From<Option<&str>> for NodeTypeDirection {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(value) => match value {
                v if v.contains("Sink") || v.contains("Input") => NodeTypeDirection::In,
                v if v.contains("Source") || v.contains("Output") => NodeTypeDirection::Out,
                _ => NodeTypeDirection::None,
            },
            None => NodeTypeDirection::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Audio,
    Video,
    Midi,
    Unknow(String),
    None,
}

impl From<Option<&str>> for Format {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(value) => match value {
                "32 bit float mono audio" => Format::Audio,
                "8 bit raw midi" => Format::Midi,
                "32 bit float RGBA video" => Format::Video,
                value => Format::Unknow(value.to_string()),
            },
            None => Format::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    In,
    Out,
}
impl From<pipewire::spa::utils::Direction> for Direction {
    fn from(value: pipewire::spa::utils::Direction) -> Self {
        if value == pipewire::spa::utils::Direction::Input {
            return Direction::In;
        }
        if value == pipewire::spa::utils::Direction::Output {
            return Direction::Out;
        }
        panic!("should not reatch");
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    pub id: u32,
    pub name: String,
    pub direction: Direction,
    pub format: Format,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkValue {
    pub id: u32,
    pub node_from: u32,
    pub node_to: u32,
    pub port_from: u32,
    pub port_to: u32,
    pub state: LinkState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LinkState {
    Error(String),
    Unlinked,
    Init,
    Negotiating,
    Allocating,
    Paused,
    Active,
}

impl From<pipewire::link::LinkState<'_>> for LinkState {
    fn from(value: pipewire::link::LinkState<'_>) -> Self {
        match value {
            pipewire::link::LinkState::Error(e) => LinkState::Error(e.to_owned()),
            pipewire::link::LinkState::Unlinked => LinkState::Unlinked,
            pipewire::link::LinkState::Init => LinkState::Init,
            pipewire::link::LinkState::Negotiating => LinkState::Negotiating,
            pipewire::link::LinkState::Allocating => LinkState::Allocating,
            pipewire::link::LinkState::Paused => LinkState::Paused,
            pipewire::link::LinkState::Active => LinkState::Active,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LinkChangeEvent {
    NodeFrom(u32),
    NodeTo(u32),
    PortFrom(u32),
    PortTo(u32),
    State(LinkState),
    Remove,
}

#[derive(Clone)]
pub struct Link {
    value: Arc<Mutex<LinkValue>>,
    broadcast: broadcast::Sender<LinkChangeEvent>,
}

impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value.lock().expect("Faile to get mutex");
        f.write_fmt(format_args!("{:#?}", value))
    }
}

impl Link {
    pub(crate) fn new(link_value: LinkValue) -> Self {
        let (broadcast, _) = broadcast::channel(25);
        Link {
            value: Arc::new(Mutex::new(link_value)),
            broadcast,
        }
    }

    pub(crate) fn apply_diff(&self, link: LinkValue) {
        {
            let mut node_value = self.value.lock().expect("Faile to get mutex");
            if node_value.node_from != link.node_from {
                node_value.node_from = link.node_from;
                let _ = self
                    .broadcast
                    .send(LinkChangeEvent::NodeFrom(link.node_from));
            }
        }
        {
            let mut node_value = self.value.lock().expect("Faile to get mutex");
            if node_value.node_to != link.node_to {
                node_value.node_to = link.node_to;
                let _ = self.broadcast.send(LinkChangeEvent::NodeTo(link.node_to));
            }
        }
        {
            let mut node_value = self.value.lock().expect("Faile to get mutex");
            if node_value.port_to != link.port_to {
                node_value.port_to = link.port_to;
                let _ = self.broadcast.send(LinkChangeEvent::PortTo(link.port_to));
            }
        }
        {
            let mut node_value = self.value.lock().expect("Faile to get mutex");
            if node_value.port_from != link.port_from {
                node_value.port_from = link.port_from;
                let _ = self
                    .broadcast
                    .send(LinkChangeEvent::PortFrom(link.port_from));
            }
        }
        self.change_state(link.state);
    }

    pub fn value(&self) -> LinkValue {
        self.value.lock().expect("Faile to get mutex").clone()
    }

    pub fn subcribe(&self) -> (LinkValue, broadcast::Receiver<LinkChangeEvent>) {
        let link = self.value.lock().expect("Faile to get mutex");
        let subscribe = self.broadcast.subscribe();
        ((*link).clone(), subscribe)
    }

    pub(crate) fn remove(&self) {
        let _ = self.broadcast.send(LinkChangeEvent::Remove);
    }

    pub(crate) fn change_state(&self, state: LinkState) {
        let mut node_value = self.value.lock().expect("Faile to get mutex");
        if node_value.state != state {
            node_value.state = state.clone();
            let _ = self.broadcast.send(LinkChangeEvent::State(state));
        }
    }
}
