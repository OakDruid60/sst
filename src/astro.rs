#![warn(missing_docs)]
//! # astro.rs
//!
//!
//! This is stuff related to how the universe is organised.

use crate::constants::MAX_SECTOR_SIZE_I8;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

// =============================
/// # AstroCoord
///
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct AstroCoord {
    id: (i8, i8, i8, i8, i8, i8),
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AstroType {
    Empty,

    Klingon,
    KilledKlingon,
    Romulan,
    KilledRomulan,

    Star,
    Planet,

    PlayerShip,
    Starbase,
}

// ======================================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct AstroObject {
    // id: (i8, i8, i8, i8, SectorType),
    coord: (i8, i8, i8, i8, i8, i8),
    t: AstroType,
}
// =====================================================================
//
impl fmt::Debug for AstroObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_compact_string())
        //f.debug_struct("Location").field("id", &self.t).finish()
    }
}
// =====================================================================
//
impl fmt::Display for AstroObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}
impl AstroObject {
    // =============================
    /// # create
    ///
    pub fn create(n: (i8, i8, i8, i8, i8, i8), ty: AstroType) -> Self {
        Self {
            coord: (n.0, n.1, n.2, n.3, n.4, n.5),
            t: ty,
        }
    }

    // =============================
    /// # to_key_string
    pub fn to_key_string(self) -> String {
        format!(
            "{}{}{}{}{}{}",
            self.coord.0, self.coord.1, self.coord.2, self.coord.3, self.coord.4, self.coord.5
        )
    }
    // =============================
    /// # to_compact_string
    pub fn to_compact_string(self) -> String {
        format!(
            "q:({},{}) s:({},{}) {:?}",
            self.coord.2, self.coord.3, self.coord.4, self.coord.5, self.t
        )
    }

    // ===========================================================================
    /// # calc_sector_distance
    ///
    /// Right triangles are good!
    pub fn calc_sector_distance(self, target: AstroObject) -> f64 {
        let strt_x = self.coord.4 as f64;
        let strt_y = self.coord.5 as f64;
        let target_x = target.coord.4 as f64;
        let target_y = target.coord.5 as f64;

        let dx2 = ((strt_x - target_x).abs()).powi(2);
        let dy2 = ((strt_y - target_y).abs()).powi(2);

        (dx2 + dy2).sqrt()
    }

    // ===========================================================================
    /// # calc_quad_distance
    ///
    /// Right triangles are good!
    pub fn calc_quad_distance(self, target: AstroObject) -> f64 {
        let strt_x = self.coord.2 as f64;
        let strt_y = self.coord.3 as f64;
        let target_x = target.coord.2 as f64;
        let target_y = target.coord.3 as f64;

        let dx2 = ((strt_x - target_x).abs()).powi(2);
        let dy2 = ((strt_y - target_y).abs()).powi(2);

        (dx2 + dy2).sqrt()
    }

    // =============================
    /// #  ret_quad_tuple
    ///
    pub fn ret_quad_tuple(self) -> (i8, i8) {
        (self.coord.2, self.coord.3)
    }

    // =============================
    /// # ret_sect_tuple
    ///
    pub fn ret_sect_tuple(self) -> (i8, i8) {
        (self.coord.4, self.coord.5)
    }

    // =======================================================
    /// # kill_enemy
    ///
    ///
    pub fn kill_enemy(&mut self) {
        let tmp_t = self.t;
        let mut tmp_s = AstroType::Empty;
        match tmp_t {
            AstroType::Klingon => {
                tmp_s = AstroType::KilledKlingon;
            }
            AstroType::Romulan => {
                tmp_s = AstroType::KilledRomulan;
            }
            _ => {}
        }
        self.t = tmp_s
    }

    // =============================
    //
    pub fn get_astro_type(self) -> AstroType {
        self.t
    }

    // =============================
    /// # is_same_quad
    ///
    pub fn is_same_quad(self, comp: &AstroObject) -> bool {
        if self.coord.2 == comp.coord.2 && self.coord.3 == comp.coord.3 {
            return true;
        }
        false
    }

    // =============================
    /// # is_same_sect_tuple
    ///
    pub fn is_same_sect_tuple(self, comp: (i8, i8)) -> bool {
        if self.coord.4 == comp.0 && self.coord.5 == comp.1 {
            return true;
        }
        false
    }

    // =============================
    /// # calc_nearby_sector_bounds
    ///
    pub fn calc_nearby_sector_bounds(self) -> ((i8, i8), (i8, i8)) {
        let mut min_x: i8 = self.coord.4 - 1;
        let mut max_x: i8 = self.coord.4 + 1;
        let mut min_y: i8 = self.coord.5 - 1;
        let mut max_y: i8 = self.coord.5 + 1;
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

    // ====================================================================
    /// # create_new_random_enterprise_in_this_quad
    ///
    pub fn create_new_random_enterprise_in_this_quad(nq: (i8, i8)) -> Self {
        Self {
            coord: (
                0,
                0,
                nq.0,
                nq.1,
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
            ),
            t: AstroType::PlayerShip,
        }
    }
}
