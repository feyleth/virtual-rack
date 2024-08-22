use o2o::o2o;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeEvent {
    pub id: u32,
    pub event: NodeChangeEvent,
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::NodeTypeDirection)]
pub enum NodeTypeDirection {
    In,
    Out,
    Both,
    None,
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::NodeChangeEvent)]
pub enum NodeChangeEvent {
    Name(String),
    State(#[from(~.into())] NodeState),
    AddPort(#[from(~.into())] Port),
    ModifyPort(#[from(~.into())] Port),
    RemovePort(u32),
    NodeType(#[from(~.into())] NodeTypeDirection),
    Remove,
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::NodeValue)]
#[serde(rename_all = "camelCase")]
pub struct NodeValue {
    pub id: u32,
    pub name: String,
    #[from(~.into())]
    pub state: NodeState,
    #[from(~.into())]
    pub media: Media,
    #[from(~.into_iter().map(|(_,item)|item.into()).collect())]
    pub ports: Vec<Port>,
    #[from(~.into())]
    pub node_type: NodeTypeDirection,
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::Format)]
pub enum Format {
    Audio,
    Video,
    Midi,
    Unknow(String),
    None,
}
#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::NodeState)]
pub enum NodeState {
    Creating,
    Suspended,
    Idle,
    Running,
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::Media)]
pub enum Media {
    Audio,
    Video,
    Midi,
    Unknow(String),
    None,
}
#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::Direction)]
pub enum Direction {
    In,
    Out,
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::Port)]
pub struct Port {
    pub id: u32,
    pub name: String,
    #[from(~.into())]
    pub direction: Direction,
    #[from(~.into())]
    pub format: Format,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkEvent {
    pub id: u32,
    pub event: LinkChangeEvent,
}
#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::LinkChangeEvent)]
pub enum LinkChangeEvent {
    NodeFrom(u32),
    NodeTo(u32),
    PortFrom(u32),
    PortTo(u32),
    State(#[from(~.into())] LinkState),
    Remove,
}
#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::LinkValue)]
pub struct LinkValue {
    pub id: u32,
    pub node_from: u32,
    pub node_to: u32,
    pub port_from: u32,
    pub port_to: u32,
    #[from(~.into())]
    pub state: LinkState,
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(crate::pipewire::node::LinkState)]
pub enum LinkState {
    Error(String),
    Unlinked,
    Init,
    Negotiating,
    Allocating,
    Paused,
    Active,
}
