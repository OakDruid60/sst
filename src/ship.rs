#![warn(missing_docs)]
//! # ship.rs
//!
//! This is stuff related to a ship (i.e. enterprise)
//!
use crate::constants::{FULL_ENTERPRISE_ENERGY, FULL_ENTERPRISE_TORPEDOES};
use crate::location::Location;

use serde::{Deserialize, Serialize};

// ================================================================
/// #ShipInfo
///  Information about the ship i.e. energy, location, torpedoes
///

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct ShipInfo {
    loc: Location,
    energy: isize,
    torpedoes: isize,
}

// ============================================================================
//
impl ShipInfo {
    // =============================
    /// #
    ///
    // pub fn create7(nsi: SectorInfo) -> ShipInfo {
    //    let ship: ShipInfo = ShipInfo {
    //        sect_info: nsi.clone(),
    //        energy: FULL_ENTERPRISE_ENERGY,
    //        torpedoes: FULL_ENTERPRISE_TORPEDOES,
    //    };
    //    ship
    //}

    // =======================================================================
    /// # get_location
    ///
    pub fn get_location(self) -> Location {
        self.loc
    }
    // =======================================================================
    /// #
    ///
    //pub fn update_sector_info(&mut self, n_info: SectorInfo) -> () {
    //    self.sect_info.set_qx(n_info.get_qx());
    //    self.sect_info.set_qy(n_info.get_qy());
    //    self.sect_info.set_sx(n_info.get_sx());
    //    self.sect_info.set_sy(n_info.get_sy());
    //}
    // =======================================================================
    /// # get_torpedoes
    ///
    pub fn get_torpedoes(self) -> isize {
        self.torpedoes
    }
    // =======================================================================
    /// # reset_torpedoes
    ///
    pub fn reset_torpedoes(&mut self) -> () {
        self.torpedoes = FULL_ENTERPRISE_TORPEDOES;
    }
    // =======================================================================
    /// # use_torpedoes
    ///
    pub fn use_torpedoe(&mut self) -> () {
        self.torpedoes -= 1;
    }

    // =======================================================================
    /// # get_energy
    ///
    pub fn get_energy(self) -> isize {
        self.energy
    }
    // =======================================================================
    /// # reset_energy
    ///
    pub fn reset_energy(&mut self) -> () {
        self.energy = FULL_ENTERPRISE_ENERGY;
    }
    // =======================================================================
    /// # use_energy
    ///
    pub fn use_energy(&mut self, n_info: isize) -> () {
        self.energy -= n_info;
    }

    pub fn set_location(&mut self, n_info: Location) -> () {
        self.loc = n_info;
    }

    // =============================
    /// # create_enterprise
    ///
    pub fn new() -> ShipInfo {
        let ship: ShipInfo = ShipInfo {
            loc: Location::new(),
            energy: FULL_ENTERPRISE_ENERGY,
            torpedoes: FULL_ENTERPRISE_TORPEDOES,
        };
        ship
    }
}
