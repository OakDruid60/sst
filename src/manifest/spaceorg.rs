#![warn(missing_docs)]
//! # spaceorg.rs
//!
//! This is stuff related to how space is organised.

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
#[derive(Clone, Copy)]
pub struct SpaceLocation {
    loc: (i8, i8Gal,
    q: Quad,
    s: Sect,
}

// ======================================================================
#[derive(Clone, Copy)]
pub struct SpaceDesignator {
    loc: (Gal,Quad,Sect),
(i8,i8),(i8,i8),(i8,i8)),
g: Gal,
    q: Quad,
    s: Sect,
}
// ======================================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Quad {
    loc: (i8, i8),
}
impl Quad {
    pub fn new(a: i8, b: i8) -> Self {
        Self { loc: (a, b) }
    }
}
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Sect {
    loc: (i8, i8),
}
impl Sect {
    pub fn new(x: i8, y: i8) -> Self {
        Self { loc: (x, y) }
    }
}

// ======================================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Entity {
    // id: (i8, i8, i8, i8, SectorType),
    q: Quad,
    s: Sect,
    t: SectorType,
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
            q: Quad::new(99 as i8, 99 as i8),
            s: Sect::new(99 as i8, 99 as i8),
            t: SectorType::Empty,
        }
    }

    // =============================
    /// # create
    ///
    pub fn create(n: (i8, i8, i8, i8, SectorType)) -> Self {
        Self {
            //    id: (n.0, n.1, n.2, n.3, n.4),
            q: Quad::new(n.0, n.1),
            s: Sect::new(n.2, n.3),
            t: n.4,
        }
    }

    // =============================
    //
    pub fn get_sector_type(self) -> SectorType {
        self.t
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
        if self.q.loc.0 == comp.q.loc.0 && self.q.loc.1 == comp.q.loc.1 {
            return true;
        }
        false
    }

    // =============================
    /// # is_same_sect
    ///
    pub fn is_same_sect(self, comp: &Entity) -> bool {
        if self.s.loc.0 == comp.s.loc.0 && self.s.loc.1 == comp.s.loc.1 {
            return true;
        }
        false
    }

    // =============================
    /// # is_same_sect_tuple
    ///
    pub fn is_same_sect_tuple(self, comp: (i8, i8)) -> bool {
        if self.s.loc.0 == comp.0 && self.s.loc.1 == comp.1 {
            return true;
        }
        false
    }

    // =============================
    /// # calc_nearby_sector_bounds
    ///
    pub fn calc_nearby_sector_bounds(self) -> ((i8, i8), (i8, i8)) {
        let mut min_x: i8 = self.s.loc.0 - 1;
        let mut max_x: i8 = self.s.loc.0 + 1;
        let mut min_y: i8 = self.s.loc.1 - 1;
        let mut max_y: i8 = self.s.loc.1 + 1;
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
    /// #  create_quad_tuple
    ///
    pub fn create_quad_tuple(self) -> (i8, i8) {
        (self.q.loc.0, self.q.loc.1)
    }

    // =============================
    /// # create_sect_tuple
    ///
    pub fn create_sect_tuple(self) -> (i8, i8) {
        (self.s.loc.0, self.s.loc.1)
    }

    // =============================
    /// # to_compact_string
    pub fn to_compact_string(self) -> String {
        return format!(
            "q:({},{}) s:({},{}) {:?}",
            self.q.loc.0, self.q.loc.1, self.s.loc.0, self.s.loc.1, self.t
        );
    }

    // =======================================================
    /// # kill_enemy
    ///
    ///
    pub fn kill_enemy(&mut self) {
        let tmp_t = self.t;
        let mut tmp_s = SectorType::Empty;
        match tmp_t {
            SectorType::Klingon => {
                tmp_s = SectorType::KilledKlingon;
            }
            SectorType::Romulan => {
                tmp_s = SectorType::KilledRomulan;
            }
            _ => {}
        }
        self.t = tmp_s
    }

    // ===========================================================================
    /// # calc_sector_distance
    ///
    /// Right triangles are good!
    pub fn calc_sector_distance(self, target: Entity) -> f64 {
        let strt_x = self.s.loc.0 as f64;
        let strt_y = self.s.loc.1 as f64;
        let target_x = target.s.loc.0 as f64;
        let target_y = target.s.loc.1 as f64;

        let dx2 = ((strt_x - target_x).abs()).powi(2);
        let dy2 = ((strt_y - target_y).abs()).powi(2);

        (dx2 + dy2).sqrt()
    }

    // ===========================================================================
    /// # calc_quad_distance
    ///
    /// Right triangles are good!
    pub fn calc_quad_distance(self, target: Entity) -> f64 {
        let strt_x = self.q.loc.0 as f64;
        let strt_y = self.q.loc.1 as f64;
        let target_x = target.q.loc.0 as f64;
        let target_y = target.q.loc.1 as f64;

        let dx2 = ((strt_x - target_x).abs()).powi(2);
        let dy2 = ((strt_y - target_y).abs()).powi(2);

        (dx2 + dy2).sqrt()
    }

    // ====================================================================
    /// # create_new_random_enterprise_in_this_quad
    ///
    pub fn create_new_random_enterprise_in_this_quad(nq: (i8, i8)) -> Self {
        Self {
            q: Quad::new(nq.0, nq.1),
            s: Sect::new(
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
            ),
            t: SectorType::Enterprise,
        }
    }
}
// =====================================================================
//
impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Location").field("id", &self.t).finish()
    }
}
// =====================================================================
//
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}
