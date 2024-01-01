#![warn(missing_docs)]
// =====================================================================
/// #SectorType
///  The type of object inhabiting the sector at the supplied UniversalCoordinate.  Only one type at a time.
///
/// **NOTE** Sectors that are Empty should be marked accordingly.  jap66
///
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SectorType {
    Empty,

    Klingon,
    KilledKlingon,
    Romulan,
    KilledRomulan,

    Star,
    Planet,

    Enterprise,
    Starbase,
}

// =====================================================================
/// #CmdType
///  
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmdType {
    Help,
    Quit,
    SRS,
    LRS,
    Phaser,
    Torpedoe,
    Move,
    Jump,
    Test,
    Status,
    Empty,
}

// =====================================================================
/// #AlertStatus
///  
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlertStatus {
    Normal,
    Docked,
    Red,
    Yellow,
}
