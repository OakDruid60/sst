#![warn(missing_docs)]
//! # astro.rs
//!
//!
//! This is stuff related to how the universe is organised.

use crate::manifest::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
//use crate::manifest::enums::{DamageType, EntityType};
//use crate::enterprise::ShipInfo;
//use crate::manifest::statistics::SummaryStats;

use rand::Rng;
use serde::{Deserialize, Serialize};
//use serde_json::from_str;
use std::collections::HashMap;
use std::fmt;
//use std::fs::File;
//use std::io::{Read, Write};

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
        f.debug_struct("Location").field("id", &self.t).finish()
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
    /// # is_same
    ///
    pub fn is_same(self, comp: &AstroObject) -> bool {
        if self.is_same_quad(comp) && self.is_same_sect(comp) {
            return true;
        }
        false
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
    /// # is_same_sect
    ///
    pub fn is_same_sect(self, comp: &AstroObject) -> bool {
        if self.coord.4 == comp.coord.4 && self.coord.5 == comp.coord.5 {
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
}
/*

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

*/

/*

// =============================
// =============================
/// # Entity

    // =============================
    /// # to_compact_string
    pub fn to_compact_string(self) -> String {
        return format!(
            "q:({},{}) s:({},{}) {:?}",
            self.q.loc.0, self.q.loc.1, self.s.loc.0, self.s.loc.1, self.t
        );
    }

*/
/*

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

*/

// ======================================================================
// ======================================================================
/// AstroUniverse
// ======================================================================
// ======================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AstroUniverse {
    data_map: HashMap<String, AstroObject>,
}

impl AstroUniverse {
    pub fn new() -> Self {
        Self {
            data_map: HashMap::new(),
        }
    }
    // =========================================================================
    /// # construct_galaxy
    ///
    pub fn construct_galaxy(gal_coord: (i8, i8)) -> HashMap<String, AstroObject> {
        let mut n_galaxy_map: HashMap<String, AstroObject> = HashMap::new();
        let n_gal_x = gal_coord.0;
        let n_gal_y = gal_coord.1;
        //
        // set initial starbase
        let n_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        let n_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);

        let n_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
        let n_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);

        let starbase_info: AstroObject = AstroObject::create(
            (n_gal_x, n_gal_y, n_quad_x, n_quad_y, n_sect_x, n_sect_y),
            AstroType::Starbase,
        );
        n_galaxy_map.insert(starbase_info.to_key_string(), starbase_info);

        //
        // set initial enterprise
        let mut existing_collision = false;
        while !existing_collision {
            let trial_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
            let trial_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
            let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
            let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);

            let enterprise_info: AstroObject = AstroObject::create(
                (
                    n_gal_x,
                    n_gal_y,
                    trial_quad_x,
                    trial_quad_y,
                    trial_sect_x,
                    trial_sect_y,
                ),
                AstroType::PlayerShip,
            );
            let n_key = enterprise_info.to_key_string();
            existing_collision = n_galaxy_map.contains_key(&n_key);
            if !existing_collision {
                n_galaxy_map.insert(n_key, enterprise_info);
                break;
            }
        }
        //        for (key, value) in n_galaxy_map.iter() {
        //            println!("{} {:?}", key,value.get_astro_type());
        //        }
        //        println!("=============================================");

        // now populate other things into each quadrant
        for xx in 0..MAX_GALAXY_SIZE_I8 {
            for yy in 0..MAX_GALAXY_SIZE_I8 {
                let trial_quad_x: i8 = xx;
                let trial_quad_y: i8 = yy;

                // planets
                for _counter in 0..rand::thread_rng().gen_range(0..3) {
                    existing_collision = false;
                    while !existing_collision {
                        let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let n_info: AstroObject = AstroObject::create(
                            (
                                n_gal_x,
                                n_gal_y,
                                trial_quad_x,
                                trial_quad_y,
                                trial_sect_x,
                                trial_sect_y,
                            ),
                            AstroType::Planet,
                        );
                        let n_key = n_info.to_key_string();
                        existing_collision = n_galaxy_map.contains_key(&n_key);
                        if !existing_collision {
                            n_galaxy_map.insert(n_key, n_info);
                            break;
                        }
                    }
                }

                // Stars
                for _counter in 0..rand::thread_rng().gen_range(1..5) {
                    existing_collision = false;
                    while !existing_collision {
                        let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let n_info: AstroObject = AstroObject::create(
                            (
                                n_gal_x,
                                n_gal_y,
                                trial_quad_x,
                                trial_quad_y,
                                trial_sect_x,
                                trial_sect_y,
                            ),
                            AstroType::Star,
                        );
                        let n_key = n_info.to_key_string();
                        existing_collision = n_galaxy_map.contains_key(&n_key);
                        if !existing_collision {
                            n_galaxy_map.insert(n_key, n_info);
                            break;
                        }
                    }
                }

                // Klingons
                for _counter in 0..rand::thread_rng().gen_range(0..2) {
                    existing_collision = false;
                    while !existing_collision {
                        let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let n_info: AstroObject = AstroObject::create(
                            (
                                n_gal_x,
                                n_gal_y,
                                trial_quad_x,
                                trial_quad_y,
                                trial_sect_x,
                                trial_sect_y,
                            ),
                            AstroType::Klingon,
                        );
                        let n_key = n_info.to_key_string();
                        existing_collision = n_galaxy_map.contains_key(&n_key);
                        if !existing_collision {
                            n_galaxy_map.insert(n_key, n_info);
                            break;
                        }
                    }
                }

                // Romulans
                for _counter in 0..rand::thread_rng().gen_range(0..3) {
                    existing_collision = false;
                    while !existing_collision {
                        let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                        let n_info: AstroObject = AstroObject::create(
                            (
                                n_gal_x,
                                n_gal_y,
                                trial_quad_x,
                                trial_quad_y,
                                trial_sect_x,
                                trial_sect_y,
                            ),
                            AstroType::Romulan,
                        );
                        let n_key = n_info.to_key_string();
                        existing_collision = n_galaxy_map.contains_key(&n_key);
                        if !existing_collision {
                            n_galaxy_map.insert(n_key, n_info);
                            break;
                        }
                    }
                }
            }
        }
        for (key, value) in n_galaxy_map.iter() {
            //let n_info = *si;
            println!("{} {:?}", key, value.get_astro_type());
        }
        n_galaxy_map
    }
}

/*
// ======================================================================
// ======================================================================
/// GameWideData
// ======================================================================
// ======================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameWideData {
    //pub designator: GalaxyLabel,
    universe_map: HashMap<String, AstroObject>,
}
impl GameWideData {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            //charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            data_map: Vec::new(),
            test_cmds_vec: Vec::new(),
            enterprise: ShipInfo::new(),
            password: "jap".to_string(),
        }
    }

    // =========================================================================
    /// # create_quadrant_vec
    ///
    /// create a quadrant information vector for the location of interest.
    ///
    pub fn create_quadrant_vec(&self, interest_loc: AstroType) -> Vec<AstroObject> {
        let mut n_vec: Vec<AstroObject> = Vec::new();
        for si in self.galaxy_vec.iter() {
            let n_info = *si;
            if n_info.is_same_quad(&interest_loc) {
                n_vec.push(n_info);
            }
        }
        n_vec
    }

    // ==========================================================================
    /// # game_stat_create_disp_vec
    ///
    pub fn game_stat_create_disp_vec(self, create_complete: bool) -> Vec<String> {
        let input_vec: Vec<AstroObject>;
        if create_complete {
            input_vec = self.galaxy_vec;
        } else {
            input_vec = self.create_quadrant_vec(self.clone().enterprise.get_entity());
        }
        let summary: SummaryStats = crate::manifest::statistics::calculate(&input_vec);

        let mut human_out: Vec<String> = Vec::new();
        human_out.push(format!(
            "  Enterprise ┃ energy:{:<6} torp:{:<3}",
            self.enterprise.get_energy(),
            self.enterprise.get_torpedoes()
        ));
        human_out.push(format!(
            " Astro Count ┃ stars:{:<3}  planets:{:<3}",
            summary.num_stars, summary.num_planets
        ));
        human_out.push(format!(
            "     Klingon ┃ t:{:<4}  k:{:<4}  a:{:<4}",
            summary.num_killed_klingons + summary.num_alive_klingons,
            summary.num_killed_klingons,
            summary.num_alive_klingons
        ));
        human_out.push(format!(
            "     Romulan ┃ t:{:<4}  k:{:<4}  a:{:<4}",
            summary.num_killed_romulans + summary.num_alive_romulans,
            summary.num_killed_romulans,
            summary.num_alive_romulans
        ));
        human_out.push(format!(
            "       Score ┃       {:<6}          ",
            summary.cur_score
        ));

        human_out
    }
}

// =========================================================================
/// # create_vec_of_type
///
pub fn create_vec_of_type(orig_vec: &Vec<AstroObject>, n_type: AstroType) -> Vec<AstroObject> {
    let mut n_type_vec: Vec<AstroObject> = Vec::new();

    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.get_sector_type() == n_type {
            n_type_vec.push(n_info);
        }
    }

    n_type_vec
}


// ============================================================
/// # find_actual_sector_info
/// Given a vector of SectorInfo, return the given sector, or Empty if not found.
pub fn find_actual_sector_info(orig_vec: &Vec<AstroObject>, sect: (i8, i8)) -> Entity {
    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.is_same_sect_tuple(sect) {
            return n_info;
        }
    }
    Entity::new()
}

// =================================================================
/// # is_straight_line_path_clear
///
pub fn is_straight_line_path_clear(
    quad_vec: &Vec<AstroObject>,
    strt: Entity,
    tgt: Entity,
) -> Result<bool, String> {
    let strt_tuple = strt.create_sect_tuple();
    let tgt_tuple = tgt.create_sect_tuple();

    let mut delta_x: f64 = tgt_tuple.0 as f64 - strt_tuple.0 as f64;
    let mut delta_y: f64 = tgt_tuple.1 as f64 - strt_tuple.1 as f64;
    let distance: f64 = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

    delta_x = delta_x / distance;
    delta_y = delta_y / distance;

    let mut trial_loc_x = strt_tuple.0 as f64 + 0.5;
    let mut trial_loc_y = strt_tuple.1 as f64 + 0.5;

    // check path for clearance

    for _ in 0..(distance as usize) {
        trial_loc_x += delta_x;
        trial_loc_y += delta_y;
        let x7 = (trial_loc_x.floor()) as i32;
        let y7 = (trial_loc_y.floor()) as i32;
        let sector_info: Entity = find_actual_sector_info(&quad_vec, (x7 as i8, y7 as i8));
        //println!("{} {} {:?}", x7, y7, sector_info.obj_type);
        if sector_info.get_sector_type() != AstroType::PlayerShip
            && sector_info.get_sector_type() != AstroType::Empty
            && sector_info.get_sector_type() != AstroType::KilledKlingon
            && sector_info.get_sector_type() != AstroType::KilledRomulan
        {
            if x7 == tgt_tuple.0 as i32 && y7 == tgt_tuple.1 as i32 {
            } else {
                return Err(format!(
                    "Straight line path from {:?} to {:?} is blocked at {:?}",
                    strt, tgt, sector_info
                )
                .to_string());
            }
        }
    }

    return Ok(true);
}

// ==========================================================================
/// # create_bad_guy_qi_vec
///
/// create a quadrant informationm vector from the supplied quadrant vector
/// and location of interest (probably the enterprise).
pub fn create_bad_guy_qi_vec(
    qi_vec: &Vec<AstroObject>,
    interest_loc: Entity,
    chk_path: bool,
) -> Vec<AstroObject> {
    let mut n_vec: Vec<AstroObject> = Vec::new();
    for si in qi_vec.iter() {
        let n_info = *si;
        let bad_guy_type: AstroType = n_info.get_sector_type();
        if bad_guy_type == AstroType::Klingon || bad_guy_type == AstroType::Romulan {
            if chk_path {
                let path_res = is_straight_line_path_clear(&qi_vec, interest_loc, n_info);
                match path_res {
                    Ok(_) => {
                        if path_res.unwrap() {
                            n_vec.push(n_info);
                        }
                    }
                    _ => {}
                }
            } else {
                n_vec.push(n_info);
            }
        }
    }

    n_vec
}

*/
