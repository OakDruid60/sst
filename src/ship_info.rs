#![warn(missing_docs)]
//! # enterprise.rs
//!
//! Contains the ShipInfo structure.  The directory contains the movement and weapon
//! source
//!
/*
pub mod movement; // jump and move
pub mod weapon;

use crate::manifest::constants::{FULL_ENTERPRISE_ENERGY, FULL_ENTERPRISE_TORPEDOES};
use crate::manifest::entity::Entity;

use serde::{Deserialize, Serialize};

// ================================================================
/// #ShipInfo
///  Information about the ship i.e. energy, location, torpedoes
///

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct ShipInfo {
    loc: Entity,
    energy: isize,
    torpedoes: isize,
}

// ============================================================================
//
impl ShipInfo {
    // =======================================================================
    /// # get_entity
    ///
    pub fn get_entity(self) -> Entity {
        self.loc
    }

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

    pub fn set_entity(&mut self, n_info: Entity) -> () {
        self.loc = n_info;
    }

    // =============================
    /// # new
    ///
    pub fn new() -> ShipInfo {
        let ship: ShipInfo = ShipInfo {
            loc: Entity::new(),
            energy: FULL_ENTERPRISE_ENERGY,
            torpedoes: FULL_ENTERPRISE_TORPEDOES,
        };
        ship
    }
}
*/