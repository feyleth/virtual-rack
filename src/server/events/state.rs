use std::collections::HashMap;

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
    #[from(~.into_iter().map(|(index,item)|(index,item.value().into())).collect())]
    pub nodes: HashMap<u32, NodeValue>,
    #[from(~.into_iter().map(|(index,item)|(index,item.value().into())).collect())]
    pub link: HashMap<u32, LinkValue>,
}
