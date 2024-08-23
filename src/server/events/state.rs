use o2o::o2o;
use serde::{Deserialize, Serialize};

use super::node::{LinkValue, NodeValue};

// #[derive(Debug, Deserialize, Serialize, o2o)]
// #[from_owned(crate::pipewire::state::StateChangeEvent)]
// pub enum StateChangeEvent {
//     AddNode(#[from(~.value().into())] NodeValue),
//     AddLink(#[from(~.value().into())] LinkValue),
// }

#[derive(Debug, Deserialize, Serialize, o2o)]
#[from_owned(crate::pipewire::state::StateValue)]
pub struct StateValue {
    #[from(~.into_iter().map(|(_,item)|item.value().into()).collect())]
    pub nodes: Vec<NodeValue>,
    #[from(~.into_iter().map(|(_,item)|item.value().into()).collect())]
    pub links: Vec<LinkValue>,
}
