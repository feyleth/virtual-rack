#[derive(Debug)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub state: State,
    pub in_ports: Vec<Port>,
    pub out_ports: Vec<Port>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Port {
    pub id: u32,
}
