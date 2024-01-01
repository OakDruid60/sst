#![warn(missing_docs)]
//! # sector.rs
//!
//! This is stuff related to sectors

use serde::{Deserialize, Serialize};

use std::fmt;

use crate::enums::SectorType;
use crate::location::Location;

// =====================================================================
/// #SectorInfo
///  Information about the sector i.e. galactic coordinate, type
///
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SectorInfo {
    loc: Location,
    s_type: SectorType,
}
impl Default for SectorInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl SectorInfo {
    // =============================
    /// # get_location
    ///
    pub fn get_location(self) -> Location {
        self.loc
    }

    // =============================
    /// # set_location
    ///
    pub fn set_location(&mut self, n_val: Location) -> () {
        self.loc = n_val.clone();
    }

    // =============================
    /// # get_sector_type
    ///
    pub fn get_sector_type(self) -> SectorType {
        self.s_type
    }
    // =============================
    /// # set_sector_type
    ///
    pub fn set_sector_type(&mut self, n_val: SectorType) -> () {
        self.s_type = n_val;
    }

    pub fn new() -> Self {
        Self {
            loc: Location::new(),
            s_type: SectorType::Empty,
        }
    }
    // =============================
    /// # create
    ///
    pub fn create(new_loc: Location, nt: SectorType) -> Self {
        Self {
            loc: new_loc.clone(),
            s_type: nt,
        }
    }
    // =======================================================
    /// # kill_enemy
    ///
    ///
    pub fn kill_enemy(&mut self) {
        let t = self.s_type;
        let mut s = SectorType::Empty;
        match t {
            SectorType::Klingon => {
                s = SectorType::KilledKlingon;
            }
            SectorType::Romulan => {
                s = SectorType::KilledRomulan;
            }
            _ => {}
        }
        self.s_type = s
    }
}
// =====================================================================
//
impl fmt::Display for SectorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ==================================================================
//
impl fmt::Debug for SectorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectorInfo")
            .field("loc", &self.loc)
            .field("s_type", &self.s_type)
            .finish()
    }
}
