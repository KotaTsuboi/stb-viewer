use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbCommon {
    pub stb_reinforcement_strength_list: StbReinforcementStrengthList,
}

impl StbCommon {
    pub fn new() -> StbCommon {
        let map = HashMap::new();
        let stb_reinforcement_strength_list = StbReinforcementStrengthList { map };
        StbCommon {
            stb_reinforcement_strength_list,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbReinforcementStrengthList {
    map: HashMap<String, String>,
}

impl StbReinforcementStrengthList {
    pub fn insert(&mut self, d: String, sd: String) {
        self.map.insert(d, sd);
    }

    pub fn get(&self, d: String) -> Option<&String> {
        self.map.get(&d)
    }
}

/*
#[derive(Debug)]
pub struct StbReinforcementStrength<'a> {
    pub d: &'a str,
    pub sd: &'a str,
}
*/
