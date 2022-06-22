pub mod stb_axes_and_stories;
pub mod stb_members;
pub mod stb_nodes;
pub mod stb_sections;

use serde::{Deserialize, Serialize};
use stb_axes_and_stories::StbAxes;
use stb_axes_and_stories::StbStories;
use stb_members::StbMembers;
use stb_nodes::StbNodes;
use stb_sections::StbSections;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbModel {
    pub stb_nodes: StbNodes,
    pub stb_axes: StbAxes,
    pub stb_stories: StbStories,
    pub stb_members: StbMembers,
    pub stb_sections: StbSections,
}
