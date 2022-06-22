pub mod stb_common;
pub mod stb_extensions;
pub mod stb_model;

use self::stb_common::StbCommon;
use self::stb_extensions::StbExtensions;
use self::stb_model::stb_nodes::StbNode;
use self::stb_model::StbModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StBridge {
    pub version: String,
    pub stb_common: StbCommon,
    pub stb_model: StbModel,
    pub stb_extensions: StbExtensions,
}

impl StBridge {
    pub fn members(&self) -> Vec<(&StbNode, &StbNode)> {
        self.stb_model
            .stb_members
            .iter()
            .map(|m| {
                let node_i = self.stb_model.stb_nodes.get(m.node_i()).unwrap();
                let node_j = self.stb_model.stb_nodes.get(m.node_j()).unwrap();
                (node_i, node_j)
            })
            .collect()
    }
}
