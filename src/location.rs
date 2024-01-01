#![warn(missing_docs)]
//! # location.rs
//!
//! This is stuff related to the galaxy location.  It is essentially a
//! wrapper to a tuple (i8, i8, i8, i8).
//!
use crate::constants::MAX_SECTOR_SIZE_I8;
//use crate::sector::SectorInfo;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

// ======================================================================
// ======================================================================
// Location
// ======================================================================
// ======================================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Location {
    id: (i8, i8, i8, i8),
}
impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}
impl Location {
    pub fn new() -> Self {
        Self {
            id: (99, 99, 99, 99),
        }
    }

    // =============================
    /// # gen_random_loc
    ///
    //    pub fn gen_random_loc(self) -> Self {
    //        Self {
    //            id: (
    //                self.id.0,
    //                self.id.1,
    //                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
    //                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
    //            ),
    //        }
    //    }

    // =============================
    /// # create
    ///
    pub fn create(n: (i8, i8, i8, i8)) -> Self {
        Self {
            id: (n.0, n.1, n.2, n.3),
        }
    }
    /*
    pub fn create_random_uni_in_quad(
        nq: (i8, i8),
        galaxy_vec: Vec<SectorInfo>,
    ) -> UniversalCoordinate {
        let mut existing_collision: bool = false;
        let mut trial_uni_coord: UniversalCoordinate = UniversalCoordinate::default();
        'unique_chk: while !existing_collision {
            trial_uni_coord = UniversalCoordinate {
                q_coord: nq,
                s_coord: (
                    rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                    rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                ),
            };
            // let trial_si: SectorInfo = SectorInfo::create(trial_uni_coord, nt);
            'search: for si in galaxy_vec.iter() {
                if trial_uni_coord.is_same_uni_coord(si.get_uni_coord()) {
                    existing_collision = true;
                    break 'search;
                }
            }
            if !existing_collision {
                //galaxy_vec.push(trial_si);
                break 'unique_chk;
            }
        }
        trial_uni_coord
    }
    // =============================
    /// # create_from_tuples
    ///
    pub fn create_from_tuples(nq: (i8, i8), ns: (i8, i8)) -> UniversalCoordinate {
        UniversalCoordinate {
            q_coord: nq,
            s_coord: ns,
        }
    }
    */
    // =============================
    /// # is_same
    ///
    pub fn is_same(self, comp: Location) -> bool {
        if self.is_same_quad(comp) && self.is_same_sect(comp) {
            return true;
        }
        false
    }
    // =============================
    /// # is_same_quad
    ///
    pub fn is_same_quad(self, comp: Location) -> bool {
        if self.id.0 == comp.id.0 && self.id.1 == comp.id.1 {
            return true;
        }
        false
    }

    // =============================
    /// # is_same_sect
    ///
    pub fn is_same_sect(self, comp: Location) -> bool {
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
        if max_x > crate::constants::MAX_SECTOR_SIZE_I8 {
            max_x = crate::constants::MAX_SECTOR_SIZE_I8;
        }
        if min_y < 0 {
            min_y = 0;
        }
        if max_y > crate::constants::MAX_SECTOR_SIZE_I8 {
            max_y = crate::constants::MAX_SECTOR_SIZE_I8;
        }

        ((min_x, min_y), (max_x, max_y))
    }

    // =============================
    /// # gen_quad_tuple
    ///
    pub fn gen_quad_tuple(self) -> (i8, i8) {
        (self.id.0, self.id.1)
    }

    // =============================
    /// # get_sect_tuple
    ///
    pub fn gen_sect_tuple(self) -> (i8, i8) {
        (self.id.2, self.id.3)
    }
    // =============================
    /// # to_compact_string
    pub fn to_compact_string(self) -> String {
        return format!("({},{},{},{})", self.id.0, self.id.1, self.id.2, self.id.3);
    }
}
// =====================================================================
//
impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Location").field("id", &self.id).finish()
    }
}
// =====================================================================
//
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}

// ====================================================================
/// # create_random_loc_given_quad
///
pub fn create_random_loc_given_quad(nq: (i8, i8)) -> Location {
    Location {
        id: (
            nq.0,
            nq.1,
            rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
            rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
        ),
    }
}

// ===========================================================================
/// # calc_sector_distance
///
/// Right triangles are good!
pub fn calc_sector_distance(strt: Location, target: Location) -> f64 {
    let strt_x = strt.id.2 as f64;
    let strt_y = strt.id.3 as f64;
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
pub fn calc_quad_distance(strt: Location, target: Location) -> f64 {
    let strt_x = strt.id.0 as f64;
    let strt_y = strt.id.1 as f64;
    let target_x = target.id.0 as f64;
    let target_y = target.id.1 as f64;

    let dx2 = ((strt_x - target_x).abs()).powi(2);
    let dy2 = ((strt_y - target_y).abs()).powi(2);

    (dx2 + dy2).sqrt()
}
