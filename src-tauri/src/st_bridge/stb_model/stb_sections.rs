use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSections {
    pub column_s_map: HashMap<u32, StbSection>,
    pub beam_rc_map: HashMap<u32, StbSection>,
    pub beam_s_map: HashMap<u32, StbSection>,
    pub slab_rc_map: HashMap<u32, StbSection>,
    pub brace_s_map: HashMap<u32, StbSection>,
    pub stb_sec_steel: StbSecSteel,
}

impl StbSections {
    pub fn new() -> StbSections {
        StbSections {
            column_s_map: HashMap::new(),
            beam_rc_map: HashMap::new(),
            beam_s_map: HashMap::new(),
            slab_rc_map: HashMap::new(),
            brace_s_map: HashMap::new(),
            stb_sec_steel: StbSecSteel::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StbSection {
    StbSecColumnS {
        id: u32,
        name: String,
        floor: String,
        kind_column: ColumnKind,
        direction: bool,
        base_type: SteelBaseType,
        stb_sec_steel_column: StbSecSteelColumn,
    },
    StbSecBeamRC {
        id: u32,
        name: String,
        floor: String,
        kind_beam: BeamKind,
        is_foundation: bool,
        is_canti: bool,
        d_reinforcement_main: String,
        d_stirrup: String,
        d_reinforcement_web: String,
        d_bar_spacing: String,
        strength_concrete: Option<String>,
        strength_reinforcement_main: String,
        strength_reinforcement_2nd_main: Option<String>,
        strength_stirrup: String,
        strength_reinforcement_web: String,
        strength_bar_spacing: String,
        depth_cover_left: Option<f64>,
        depth_cover_right: Option<f64>,
        depth_cover_top: Option<f64>,
        depth_cover_bottom: Option<f64>,
        stb_sec_figure: StbSecFigureBeam,
        stb_sec_bar_arrangement: StbSecBarArrangementBeam,
    },
    StbSecBeamS {
        id: u32,
        name: String,
        floor: String,
        kind_beam: BeamKind,
        is_canti: bool,
        stb_sec_steel_beam: StbSecSteelBeam,
    },
    StbSecSlabRC {
        id: u32,
        name: String,
        is_foundation: bool,
        is_canti: bool,
        strength_concrete: String,
        stb_sec_figure: StbSecFigureSlab,
        stb_sec_bar_arrangement: StbSecBarArrangementSlab,
    },
    StbSecBraceS {
        id: u32,
        name: String,
        floor: String,
        kind_brace: BraceKind,
        stb_sec_steel_brace: StbSecSteelBrace,
    },
}

impl StbSection {
    pub fn id(&self) -> u32 {
        match *self {
            StbSection::StbSecColumnS { id, .. } => id,
            StbSection::StbSecBeamRC { id, .. } => id,
            StbSection::StbSecBeamS { id, .. } => id,
            StbSection::StbSecSlabRC { id, .. } => id,
            StbSection::StbSecBraceS { id, .. } => id,
        }
    }
}

/*
pub trait StbSectionsChildren {}

impl std::fmt::Debug for dyn StbSectionsChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", "section")
    }
}
*/

//impl StbSectionsChildren for StbSecColumnS {}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum ColumnKind {
    #[strum(serialize = "COLUMN")]
    Column,
    #[strum(serialize = "POST")]
    Post,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SteelBaseType {
    #[strum(serialize = "")]
    Null,
    #[strum(serialize = "EXPOSE")]
    Expose,
    #[strum(serialize = "EMBEDDED")]
    Embedded,
    #[strum(serialize = "WRAP")]
    Wrap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecSteelColumn {
    pub pos: StbSecSteelColumnPosition,
    pub shape: String,
    pub strength_main: String,
    pub strength_web: String,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecSteelColumnPosition {
    #[strum(serialize = "ALL")]
    All,
}

//impl StbSectionsChildren for StbSecBeamRC {}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum BeamKind {
    #[strum(serialize = "GIRDER")]
    Girder,
    #[strum(serialize = "BEAM")]
    Beam,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecFigureBeam {
    pub stb_sec_haunch: Option<StbSecHaunch>,
    pub stb_sec_straight: Option<StbSecStraightBeam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecHaunch {
    pub width_start: f64,
    pub depth_start: f64,
    pub width_center: f64,
    pub depth_center: f64,
    pub width_end: f64,
    pub depth_end: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecStraightBeam {
    pub depth: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecBarArrangementBeam {
    pub stb_sec_beam_start_center_end_section_list: Option<Vec<StbSecBeamStartCenterEndSection>>,
    pub stb_sec_beam_same_section: Option<StbSecBeamSameSection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecBeamStartCenterEndSection {
    pub pos: StbSecBeamSectionPosition,
    pub count_main_top_1st: u32,
    pub count_main_bottom_1st: u32,
    pub count_stirrup: u32,
    pub pitch_stirrup: f64,
    pub count_web: u32,
    pub count_bar_spacing: u32,
    pub pitch_bar_spacing: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecBeamSameSection {
    pub count_main_top_1st: u32,
    pub count_main_bottom_1st: u32,
    pub count_stirrup: u32,
    pub pitch_stirrup: f64,
    pub count_web: u32,
    pub count_bar_spacing: u32,
    pub pitch_bar_spacing: f64,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecBeamSectionPosition {
    #[strum(serialize = "START")]
    Start,
    #[strum(serialize = "CENTER")]
    Center,
    #[strum(serialize = "END")]
    End,
}

//impl StbSectionsChildren for StbSecBeamS {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecSteelBeam {
    pub pos: StbSecSteelBeamPosition,
    pub shape: String,
    pub strength_main: String,
    pub strength_web: String,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecSteelBeamPosition {
    #[strum(serialize = "ALL")]
    All,
}

//impl StbSectionsChildren for StbSecSlabRC {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecFigureSlab {
    pub stb_sec_straight: StbSecStraightSlab,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecStraightSlab {
    pub depth: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecBarArrangementSlab {
    pub stb_sec_1way_slab_1_list: Vec<StbSec1WaySlab1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSec1WaySlab1 {
    pub pos: StbSec1WaySlab1Position,
    pub strength: String,
    pub d: String,
    pub pitch: f64,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSec1WaySlab1Position {
    #[strum(serialize = "MAIN_TOP")]
    MainTop,
    #[strum(serialize = "MAIN_BOTTOM")]
    MainBottom,
    #[strum(serialize = "TRANSVERS_TOP")]
    TransverseTop,
    #[strum(serialize = "TRANSVERS_BOTTOM")]
    TransverseBottom,
}

//impl StbSectionsChildren for StbSecBraceS {}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum BraceKind {
    #[strum(serialize = "VERTICAL")]
    Vertical,
    #[strum(serialize = "HORIZONTAL")]
    Horizontal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecSteelBrace {
    pub pos: StbSecSteelBraceSPosition,
    pub shape: String,
    pub strength_main: String,
    pub strength_web: String,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecSteelBraceSPosition {
    #[strum(serialize = "ALL")]
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StbSecSteel {
    pub children_map: HashMap<String, StbSecSteelChildren>,
    //pub roll_h_map: HashMap<String, StbSecRollH>,
    //pub build_h_map: HashMap<String, StbSecBuildH>,
    //pub roll_box_map: HashMap<String, StbSecRollBox>,
    //pub build_box_map: HashMap<String, StbSecBuildBox>,
    //pub pipe_map: HashMap<String, StbSecPipe>,
    //pub roll_l_map: HashMap<String, StbSecRollL>,
}

impl StbSecSteel {
    pub fn new() -> StbSecSteel {
        StbSecSteel {
            children_map: HashMap::new(),
            //roll_h_map: HashMap::new(),
            //build_h_map: HashMap::new(),
            //roll_box_map: HashMap::new(),
            //build_box_map: HashMap::new(),
            //pipe_map: HashMap::new(),
            //roll_l_map: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StbSecSteelChildren {
    StbSecRollH {
        name: String,
        sec_type: StbSecRollHType,
        a: f64,
        b: f64,
        t1: f64,
        t2: f64,
        r: f64,
    },
    StbSecBuildH {
        name: String,
        a: f64,
        b: f64,
        t1: f64,
        t2: f64,
    },
    StbSecRollBox {
        name: String,
        sec_type: StbSecRollBoxType,
        a: f64,
        b: f64,
        t: f64,
        r: f64,
    },
    StbSecBuildBox {
        name: String,
        a: f64,
        b: f64,
        t1: f64,
        t2: f64,
    },
    StbSecPipe {
        name: String,
        d: f64,
        t: f64,
    },
    StbSecRollL {
        name: String,
        sec_type: StbSecRollLType,
        a: f64,
        b: f64,
        t1: f64,
        t2: f64,
        r1: f64,
        r2: f64,
        side: bool,
    },
}

impl StbSecSteelChildren {
    pub fn name(&self) -> String {
        match self {
            StbSecSteelChildren::StbSecRollH { name, .. } => name.clone(),
            StbSecSteelChildren::StbSecBuildH { name, .. } => name.clone(),
            StbSecSteelChildren::StbSecRollBox { name, .. } => name.clone(),
            StbSecSteelChildren::StbSecBuildBox { name, .. } => name.clone(),
            StbSecSteelChildren::StbSecPipe { name, .. } => name.clone(),
            StbSecSteelChildren::StbSecRollL { name, .. } => name.clone(),
        }
    }

    pub fn shape(&self) -> Vec<(f64, f64)> {
        match self {
            StbSecSteelChildren::StbSecRollH { a, b, t1, t2, .. } => {
                vec![
                    (-b / 2.0, -a / 2.0),
                    (b / 2.0, -a / 2.0),
                    (b / 2.0, -a / 2.0 + t1),
                    (t2 / 2.0, -a / 2.0 + t1),
                    (t2 / 2.0, a / 2.0 - t1),
                    (b / 2.0, a / 2.0 - t1),
                    (b / 2.0, a / 2.0),
                    (-b / 2.0, a / 2.0),
                    (-b / 2.0, a / 2.0 - t1),
                    (-t2 / 2.0, a / 2.0 - t1),
                    (-t2 / 2.0, -a / 2.0 + t1),
                    (-b / 2.0, -a / 2.0 + t1),
                ]
            }
            StbSecSteelChildren::StbSecBuildH { a, b, t1, t2, .. } => {
                vec![(0.0, 0.0)]
            }
            StbSecSteelChildren::StbSecRollBox { a, b, t, .. } => {
                vec![(0.0, 0.0)]
            }
            StbSecSteelChildren::StbSecBuildBox { a, b, t1, t2, .. } => {
                vec![(0.0, 0.0)]
            }
            StbSecSteelChildren::StbSecPipe { d, t, .. } => {
                vec![(0.0, 0.0)]
            }
            StbSecSteelChildren::StbSecRollL { a, b, t1, t2, .. } => {
                vec![(0.0, 0.0)]
            }
        }
    }
}

/*
pub trait StbSecSteelChildren {}

impl std::fmt::Debug for dyn StbSecSteelChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", "steel section")
    }
}
*/

//impl StbSecSteelChildren for StbSecRollH {}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecRollHType {
    #[strum(serialize = "H")]
    H,
    #[strum(serialize = "SH")]
    SH,
}

//impl StbSecSteelChildren for StbSecBuildH {}

//impl StbSecSteelChildren for StbSecRollBox {}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecRollBoxType {
    #[strum(serialize = "BCP")]
    BCP,
    #[strum(serialize = "BCR")]
    BCR,
    #[strum(serialize = "STKR")]
    STKR,
    #[strum(serialize = "ELSE")]
    Else,
}

//impl StbSecSteelChildren for StbSecBuildBox {}

//impl StbSecSteelChildren for StbSecPipe {}

//impl StbSecSteelChildren for StbSecRollL {}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StbSecRollLType {
    #[strum(serialize = "L")]
    L,
}
