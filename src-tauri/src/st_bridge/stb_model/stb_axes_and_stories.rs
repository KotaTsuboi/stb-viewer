use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbAxes {
    pub stb_x_axis_list: Vec<StbXAxis>,
    pub stb_y_axis_list: Vec<StbYAxis>,
}

impl StbAxes {
    pub fn new() -> StbAxes {
        StbAxes {
            stb_x_axis_list: Vec::new(),
            stb_y_axis_list: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbXAxis {
    pub id: i32,
    pub name: String,
    pub distance: f64,
    pub stb_node_id_list: StbNodeIdList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbYAxis {
    pub id: i32,
    pub name: String,
    pub distance: f64,
    pub stb_node_id_list: StbNodeIdList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbNodeIdList {
    pub children: Vec<StbNodeId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbNodeId {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStories {
    pub stb_story_list: Vec<StbStory>,
}

impl StbStories {
    pub fn new() -> StbStories {
        StbStories {
            stb_story_list: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbStory {
    pub id: i32,
    pub name: String,
    pub height: f64,
    pub kind: StbStoryKind,
    pub concrete_strength: String,
    pub stb_node_id_list: StbNodeIdList,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbStoryKind {
    #[strum(serialize = "GENERAL")]
    General,
    #[strum(serialize = "BASEMENT")]
    Basement,
    #[strum(serialize = "ROOF")]
    Roof,
    #[strum(serialize = "PENTHOUSE")]
    Penthouse,
    #[strum(serialize = "ISOLATION")]
    Isolation,
    #[strum(serialize = "DEPENDENCE")]
    Dependence,
}
