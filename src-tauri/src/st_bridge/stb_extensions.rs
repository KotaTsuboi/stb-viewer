use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StbExtensions {
    pub stb_extension_list: Vec<StbExtension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbExtension {
    pub identifier: String,
    pub description: String,
}
