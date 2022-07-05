use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbNodes {
    pub map: HashMap<u32, StbNode>,
}

impl StbNodes {
    pub fn new() -> StbNodes {
        StbNodes {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: u32, value: StbNode) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: u32) -> Option<StbNode> {
        self.map.get(&key).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StbNode {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub kind: StbNodeKind,
    pub id_member: Option<u32>,
}

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
pub enum StbNodeKind {
    #[strum(serialize = "ON_GIRDER")]
    OnGirder,
    #[strum(serialize = "ON_BEAM")]
    OnBeam,
    #[strum(serialize = "ON_COLUMN")]
    OnColumn,
    #[strum(serialize = "ON_POST")]
    OnPost,
    #[strum(serialize = "ON_GRID")]
    OnGrid,
    #[strum(serialize = "ON_CANTI")]
    OnCanti,
    #[strum(serialize = "ON_SLAB")]
    OnSlab,
    #[strum(serialize = "OTHER")]
    Other,
}
