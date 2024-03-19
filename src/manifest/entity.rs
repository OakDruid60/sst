#![warn(missing_docs)]
//! # entity.rs
//!
//! This is stuff related to the entire game.

use crate::manifest::constants::MAX_SECTOR_SIZE_I8;
use crate::manifest::enums::SectorType;
//use crate::enterprise::ShipInfo;
//use crate::manifest::statistics::SummaryStats;

use rand::Rng;
use serde::{Deserialize, Serialize};
//use serde_json::from_str;
use std::fmt;
//use std::fs::File;
//use std::io::{Read, Write};

// ======================================================================
// ======================================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Entity {
    id: (i8, i8, i8, i8, SectorType),
}
impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}
impl Entity {
    // =============================
    pub fn new() -> Self {
        Self {
            id: (99, 99, 99, 99, SectorType::Empty),
        }
    }

    // =============================
    /// # create
    ///
    pub fn create(n: (i8, i8, i8, i8, SectorType)) -> Self {
        Self {
            id: (n.0, n.1, n.2, n.3, n.4),
        }
    }

    // =============================
    //
    pub fn get_sector_type(self) -> SectorType {
        self.id.4
    }

    // =============================
    //
    //pub fn set_sector_type(mut self, n_type: SectorType) -> () {
    //    self.id.4 = n_type.clone();
    //}
    // =============================
    /// # is_same
    ///
    pub fn is_same(self, comp: &Entity) -> bool {
        if self.is_same_quad(comp) && self.is_same_sect(comp) {
            return true;
        }
        false
    }
    // =============================
    /// # is_same_quad
    ///
    pub fn is_same_quad(self, comp: &Entity) -> bool {
        if self.id.0 == comp.id.0 && self.id.1 == comp.id.1 {
            return true;
        }
        false
    }

    // =============================
    /// # is_same_sect
    ///
    pub fn is_same_sect(self, comp: &Entity) -> bool {
        if self.id.2 == comp.id.2 && self.id.3 == comp.id.3 {
            return true;
        }
        false
    }

    // =============================
    /// # is_same_sect_tuple
    ///
    pub fn is_same_sect_tuple(self, comp: (i8, i8)) -> bool {
        if self.id.2 == comp.0 && self.id.3 == comp.1 {
            return true;
        }
        false
    }

    // =============================
    /// # calc_nearby_sector_bounds
    ///
    pub fn calc_nearby_sector_bounds(self) -> ((i8, i8), (i8, i8)) {
        let mut min_x: i8 = self.id.2 - 1;
        let mut max_x: i8 = self.id.2 + 1;
        let mut min_y: i8 = self.id.3 - 1;
        let mut max_y: i8 = self.id.3 + 1;
        // find an empty sector for docking

        if min_x < 0 {
            min_x = 0;
        }
        if max_x > MAX_SECTOR_SIZE_I8 {
            max_x = MAX_SECTOR_SIZE_I8;
        }
        if min_y < 0 {
            min_y = 0;
        }
        if max_y > MAX_SECTOR_SIZE_I8 {
            max_y = MAX_SECTOR_SIZE_I8;
        }

        ((min_x, min_y), (max_x, max_y))
    }

    // =============================
    /// # gen_quad_tuple
    ///
    pub fn create_quad_tuple(self) -> (i8, i8) {
        (self.id.0, self.id.1)
    }

    // =============================
    /// # get_sect_tuple
    ///
    pub fn create_sect_tuple(self) -> (i8, i8) {
        (self.id.2, self.id.3)
    }

    // =============================
    /// # to_compact_string
    pub fn to_compact_string(self) -> String {
        return format!(
            "({},{},{},{},{:?})",
            self.id.0, self.id.1, self.id.2, self.id.3, self.id.4
        );
    }

    // =======================================================
    /// # kill_enemy
    ///
    ///
    pub fn kill_enemy(&mut self) {
        let t = self.id.4;
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
        self.id.4 = s
    }

    // ===========================================================================
    /// # calc_sector_distance
    ///
    /// Right triangles are good!
    pub fn calc_sector_distance(self, target: Entity) -> f64 {
        let strt_x = self.id.2 as f64;
        let strt_y = self.id.3 as f64;
        let target_x = target.id.2 as f64;
        let target_y = target.id.3 as f64;

        let dx2 = ((strt_x - target_x).abs()).powi(2);
        let dy2 = ((strt_y - target_y).abs()).powi(2);

        (dx2 + dy2).sqrt()
    }

    // ===========================================================================
    /// # calc_quad_distance
    ///
    /// Right triangles are good!
    pub fn calc_quad_distance(self, target: Entity) -> f64 {
        let strt_x = self.id.0 as f64;
        let strt_y = self.id.1 as f64;
        let target_x = target.id.0 as f64;
        let target_y = target.id.1 as f64;

        let dx2 = ((strt_x - target_x).abs()).powi(2);
        let dy2 = ((strt_y - target_y).abs()).powi(2);

        (dx2 + dy2).sqrt()
    }

    // ====================================================================
    /// # create_new_random_enterprise_in_this_quad
    ///
    pub fn create_new_random_enterprise_in_this_quad(nq: (i8, i8)) -> Self {
        Self {
            id: (
                nq.0,
                nq.1,
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                SectorType::Enterprise,
            ),
        }
    }
}
// =====================================================================
//
impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Location").field("id", &self.id).finish()
    }
}
// =====================================================================
//
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}
