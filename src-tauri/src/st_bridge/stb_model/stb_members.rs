use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbMembers {
    pub stb_columns: HashMap<u32, StbMember>,
    pub stb_posts: HashMap<u32, StbMember>,
    pub stb_girders: HashMap<u32, StbMember>,
    pub stb_beams: HashMap<u32, StbMember>,
    pub stb_braces: HashMap<u32, StbMember>,
    pub stb_slabs: HashMap<u32, StbMember>,
    // TODO: implement StbWalls
    // TODO: implement StbFootings
    // TODO: implement StbStripFootings
    // TODO: implement StbPiles
    // TODO: implement StbFoundationColumns
    // TODO: implement StbParapets
    // TODO: implement StbOpens
}

impl StbMembers {
    pub fn new() -> StbMembers {
        StbMembers {
            stb_columns: HashMap::new(),
            stb_posts: HashMap::new(),
            stb_girders: HashMap::new(),
            stb_beams: HashMap::new(),
            stb_braces: HashMap::new(),
            stb_slabs: HashMap::new(),
        }
    }

    pub fn iter<'a>(&'a self) -> StbMembersIter<'a> {
        StbMembersIter::new(self)
    }
}

pub struct StbMembersIter<'a> {
    columns_iter: Iter<'a, u32, StbMember>,
    posts_iter: Iter<'a, u32, StbMember>,
    girders_iter: Iter<'a, u32, StbMember>,
    beams_iter: Iter<'a, u32, StbMember>,
    braces_iter: Iter<'a, u32, StbMember>,
    slabs_iter: Iter<'a, u32, StbMember>,
}

impl StbMembersIter<'_> {
    pub fn new(stb_members: &StbMembers) -> StbMembersIter {
        StbMembersIter {
            columns_iter: stb_members.stb_columns.iter(),
            posts_iter: stb_members.stb_posts.iter(),
            girders_iter: stb_members.stb_girders.iter(),
            beams_iter: stb_members.stb_beams.iter(),
            braces_iter: stb_members.stb_braces.iter(),
            slabs_iter: stb_members.stb_slabs.iter(),
        }
    }
}

impl<'a> Iterator for StbMembersIter<'a> {
    type Item = &'a StbMember;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.columns_iter.next() {
            return Some(t.1);
        } else if let Some(t) = self.posts_iter.next() {
            return Some(t.1);
        } else if let Some(t) = self.girders_iter.next() {
            return Some(t.1);
        } else if let Some(t) = self.beams_iter.next() {
            return Some(t.1);
        } else if let Some(t) = self.braces_iter.next() {
            return Some(t.1);
        } else if let Some(t) = self.slabs_iter.next() {
            return Some(t.1);
        } else {
            return None;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StbMember {
    StbColumn {
        id: u32,
        name: String,
        id_node_bottom: u32,
        id_node_top: u32,
        rotate: f64,
        id_section: u32,
        kind_structure: ColumnStructureKind,
        offset_x: f64,
        offset_y: f64,
        condition_bottom: JointCondition,
        condition_top: JointCondition,
    },
    StbPost {
        id: u32,
        name: String,
        id_node_bottom: u32,
        id_node_top: u32,
        rotate: f64,
        id_section: u32,
        kind_structure: ColumnStructureKind,
        offset_x: f64,
        offset_y: f64,
        offset_bottom_x: f64,
        offset_bottom_y: f64,
        offset_bottom_z: f64,
        offset_top_x: f64,
        offset_top_y: f64,
        offset_top_z: f64,
        condition_bottom: JointCondition,
        condition_top: JointCondition,
    },
    StbGirder {
        id: u32,
        name: String,
        id_node_start: u32,
        id_node_end: u32,
        rotate: f64,
        id_section: u32,
        kind_structure: GirderStructureKind,
        is_foundation: bool,
        offset: f64,
        level: f64,
        type_haunch_h: Option<HaunchType>,
    },
    StbBeam {
        id: u32,
        name: String,
        id_node_start: u32,
        id_node_end: u32,
        rotate: f64,
        id_section: u32,
        kind_structure: GirderStructureKind,
        is_foundation: bool,
        offset: f64,
        level: f64,
    },
    StbBrace {
        id: u32,
        name: String,
        id_node_start: u32,
        id_node_end: u32,
        rotate: f64,
        id_section: u32,
        kind_structure: BraceStructureKind,
        offset_start_x: f64,
        offset_start_y: f64,
        offset_start_z: f64,
        offset_end_x: f64,
        offset_end_y: f64,
        offset_end_z: f64,
        condition_start: JointCondition,
        condition_end: JointCondition,
    },
    StbSlab {
        id: u32,
        name: String,
        id_section: u32,
        kind_structure: SlabStructureKind,
        kind_slab: SlabKind,
        level: f64,
        is_foundation: bool,
    },
}

impl StbMember {
    pub fn id(&self) -> u32 {
        match *self {
            StbMember::StbColumn { id, .. } => id,
            StbMember::StbSlab { id, .. } => id,
            StbMember::StbBrace { id, .. } => id,
            StbMember::StbBeam { id, .. } => id,
            StbMember::StbGirder { id, .. } => id,
            StbMember::StbPost { id, .. } => id,
        }
    }

    pub fn node_i(&self) -> u32 {
        match *self {
            StbMember::StbColumn { id_node_bottom, .. } => id_node_bottom,
            StbMember::StbSlab { .. } => panic!(""),
            StbMember::StbBrace { id_node_start, .. } => id_node_start,
            StbMember::StbBeam { id_node_start, .. } => id_node_start,
            StbMember::StbGirder { id_node_start, .. } => id_node_start,
            StbMember::StbPost { id_node_bottom, .. } => id_node_bottom,
        }
    }

    pub fn node_j(&self) -> u32 {
        match *self {
            StbMember::StbColumn { id_node_top, .. } => id_node_top,
            StbMember::StbSlab { .. } => panic!(""),
            StbMember::StbBrace { id_node_end, .. } => id_node_end,
            StbMember::StbBeam { id_node_end, .. } => id_node_end,
            StbMember::StbGirder { id_node_end, .. } => id_node_end,
            StbMember::StbPost { id_node_top, .. } => id_node_top,
        }
    }
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbColumns {
    pub stb_column_list: Vec<StbColumn>,
}

impl StbColumns {
    pub fn new() -> StbColumns {
        StbColumns {
            stb_column_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum ColumnStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
    #[strum(serialize = "CFT")]
    CFT,
    #[strum(serialize = "UNDEFINED")]
    Undefined,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum JointCondition {
    #[strum(serialize = "FIX")]
    Fix,
    #[strum(serialize = "PIN")]
    Pin,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbGirders {
    pub stb_girder_list: Vec<StbGirder>,
}

impl StbGirders {
    pub fn new() -> StbGirders {
        StbGirders {
            stb_girder_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum GirderStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
    #[strum(serialize = "UNDEFINED")]
    Undefined,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum HaunchType {
    #[strum(serialize = "BOTH")]
    Both,
    #[strum(serialize = "RIGHT")]
    Right,
    #[strum(serialize = "LEFT")]
    Left,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbPosts {
    pub stb_post_list: Vec<StbPost>,
}

impl StbPosts {
    pub fn new() -> StbPosts {
        StbPosts {
            stb_post_list: Vec::new(),
        }
    }
}
*/

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbBeams {
    pub stb_beam_list: Vec<StbBeam>,
}

impl StbBeams {
    pub fn new() -> StbBeams {
        StbBeams {
            stb_beam_list: Vec::new(),
        }
    }
}
*/

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbSlabs {
    pub stb_slab_list: Vec<StbSlab>,
}

impl StbSlabs {
    pub fn new() -> StbSlabs {
        StbSlabs {
            stb_slab_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SlabStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "DECK")]
    Deck,
    #[strum(serialize = "PRECAST")]
    Precast,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SlabKind {
    #[strum(serialize = "NORMAL")]
    Normal,
    #[strum(serialize = "CANTI")]
    Canti,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbBraces {
    pub stb_brace_list: Vec<StbBrace>,
}

impl StbBraces {
    pub fn new() -> StbBraces {
        StbBraces {
            stb_brace_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum BraceStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
}
