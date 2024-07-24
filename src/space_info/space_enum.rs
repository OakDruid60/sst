#![warn(missing_docs)]
//! # enums.rs
// =====================================================================
/// #SectorType
///  The type of object inhabiting the sector at the supplied Location.
///  Only one type at a time.
///
/// **NOTE** Sectors that are Empty should be marked accordingly.
///
use serde::{Deserialize, Serialize};


// =====================================================================
/// #CmdType
///  The type of commands
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
    Restore,
    Save,
    RecordOn,
    RecordOff,
    Empty,
}

// =====================================================================
/// #AlertStatus
///  The alert status types
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlertStatus {
    Normal,
    Docked,
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Affiliation {
    Empty,
    Neutral,
    Enemy,
    Friend,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DamageType {
    Ll,
    Kk,
}
