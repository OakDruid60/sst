#![warn(missing_docs)]
//! # gamedata.rs
//!
//! This is stuff related to the entire game.
//!
//!
use crate::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::enums::SectorType;
use crate::location::Location;
use crate::sector::SectorInfo;
use crate::ship::ShipInfo;
use crate::statistics::SummaryStats;

use rand::Rng;
use serde::{Deserialize, Serialize};

// ======================================================================
// ======================================================================
// GameWideData
// ======================================================================
// ======================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameWideData {
    pub cur_star_date: i32,
    pub end_star_date: i32,
    pub charted: [[bool; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
    pub gal_vec: Vec<SectorInfo>,
    pub enterprise: ShipInfo,
}

impl GameWideData {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            gal_vec: Vec::new(),
            enterprise: ShipInfo::new(),
        }
    }

    // =========================================================================
    /// # create_quadrant_vec
    ///
    /// create a quadrant information vector for location of interest
    /// of interest.
    pub fn create_quadrant_vec(&self, interest_loc: Location) -> Vec<SectorInfo> {
        let mut n_vec: Vec<SectorInfo> = Vec::new();
        for si in self.gal_vec.iter() {
            let n_info = *si;
            if n_info.get_location().is_same_quad(interest_loc) {
                n_vec.push(n_info);
            }
        }
        n_vec
    }

    // ==========================================================================
    /// # game_stat_create_disp_vec
    ///
    pub fn game_stat_create_disp_vec(self, create_complete: bool) -> Vec<String> {
        let input_vec: Vec<SectorInfo>;
        if create_complete {
            input_vec = self.gal_vec;
        } else {
            input_vec = self.create_quadrant_vec(self.clone().enterprise.get_location());
        }
        let summary: SummaryStats = crate::statistics::calculate(&input_vec);

        let mut human_out: Vec<String> = Vec::new();
        human_out.push(format!(
            "    Enterprise |  NRG:{:<6}     torp:{}     loc:{}",
            self.enterprise.get_energy(),
            self.enterprise.get_torpedoes(),
            self.enterprise.get_location().to_compact_string()
        ));
        human_out.push(format!(
            "   Astro Count |  stars:{:<3}   planets:{}",
            summary.num_stars, summary.num_planets
        ));
        human_out.push(format!(
            "       Klingon |  t:{:<4}   k:{:<4}   a:{:<4}",
            summary.num_killed_klingons + summary.num_alive_klingons,
            summary.num_killed_klingons,
            summary.num_alive_klingons
        ));
        human_out.push(format!(
            "       Romulan |  t:{:<4}   k:{:<4}   a:{:<4}",
            summary.num_killed_romulans + summary.num_alive_romulans,
            summary.num_killed_romulans,
            summary.num_alive_romulans
        ));

        human_out
    }
}

// =========================================================================
/// # create_vec_of_type
///
pub fn create_vec_of_type(orig_vec: &Vec<SectorInfo>, n_type: SectorType) -> Vec<SectorInfo> {
    let mut n_type_vec: Vec<SectorInfo> = Vec::new();

    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.get_sector_type() == n_type {
            n_type_vec.push(n_info);
        }
    }

    n_type_vec
}

// =========================================================================
/// # construct_galaxy
///
pub fn construct_galaxy() -> Vec<SectorInfo> {
    let mut n_galaxy_vec: Vec<SectorInfo> = Vec::new();

    //
    // set initial starbase
    let n_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
    let n_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);

    let n_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
    let n_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);

    let starbase_info: SectorInfo = SectorInfo::create(
        Location::create((n_quad_x, n_quad_y, n_sect_x, n_sect_y)),
        SectorType::Starbase,
    );
    n_galaxy_vec.push(starbase_info);

    //
    // set initial enterprise
    let mut existing_collision = false;
    while !existing_collision {
        let trial_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        let trial_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
        let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
        let trial_uni_coord =
            Location::create((trial_quad_x, trial_quad_y, trial_sect_x, trial_sect_y));
        let enterprise_info: SectorInfo =
            SectorInfo::create(trial_uni_coord, SectorType::Enterprise);
        for si in n_galaxy_vec.iter() {
            if enterprise_info.get_location().is_same(si.get_location()) {
                existing_collision = true;
                break;
            }
        }
        if !existing_collision {
            n_galaxy_vec.push(enterprise_info);
            break;
        }
    }

    // now populate other things into each quadrant
    for xx in 0..MAX_GALAXY_SIZE_I8 {
        for yy in 0..MAX_GALAXY_SIZE_I8 {
            let trial_quad_x: i8 = xx as i8;
            let trial_quad_y: i8 = yy as i8;

            // planets
            for _counter in 0..rand::thread_rng().gen_range(0..3) {
                existing_collision = false;
                while !existing_collision {
                    let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                    let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                    let n_info: SectorInfo = SectorInfo::create(
                        Location::create((trial_quad_x, trial_quad_y, trial_sect_x, trial_sect_y)),
                        SectorType::Planet,
                    );
                    for si in n_galaxy_vec.iter() {
                        if n_info.get_location().is_same(si.get_location()) {
                            existing_collision = true;
                            break;
                        }
                    }
                    if !existing_collision {
                        n_galaxy_vec.push(n_info);
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
                    let n_info: SectorInfo = SectorInfo::create(
                        Location::create((trial_quad_x, trial_quad_y, trial_sect_x, trial_sect_y)),
                        SectorType::Star,
                    );
                    for si in n_galaxy_vec.iter() {
                        if n_info.get_location().is_same(si.get_location()) {
                            existing_collision = true;
                            break;
                        }
                    }
                    if !existing_collision {
                        n_galaxy_vec.push(n_info);
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
                    let n_info: SectorInfo = SectorInfo::create(
                        Location::create((trial_quad_x, trial_quad_y, trial_sect_x, trial_sect_y)),
                        SectorType::Klingon,
                    );
                    for si in n_galaxy_vec.iter() {
                        if n_info.get_location().is_same(si.get_location()) {
                            existing_collision = true;
                            break;
                        }
                    }
                    if !existing_collision {
                        n_galaxy_vec.push(n_info);
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
                    let n_info: SectorInfo = SectorInfo::create(
                        Location::create((trial_quad_x, trial_quad_y, trial_sect_x, trial_sect_y)),
                        SectorType::Romulan,
                    );
                    for si in n_galaxy_vec.iter() {
                        if n_info.get_location().is_same(si.get_location()) {
                            existing_collision = true;
                            break;
                        }
                    }
                    if !existing_collision {
                        n_galaxy_vec.push(n_info);
                        break;
                    }
                }
            }
        }
    }
    n_galaxy_vec
}

// ============================================================
/// # find_actual_sector_info
/// Given a vector of SectorInfo, return the given sector, or Empty if not found.
pub fn find_actual_sector_info(orig_vec: &Vec<SectorInfo>, sect: (i8, i8)) -> SectorInfo {
    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.get_location().is_same_sect_tuple(sect) {
            return n_info;
        }
    }
    SectorInfo::new()
}

// =================================================================
/// # is_straight_line_path_clear
///
pub fn is_straight_line_path_clear(
    quad_vec: &Vec<SectorInfo>,
    strt: Location,
    tgt: Location,
) -> Result<bool, String> {
    //let quad_vec: Vec<SectorInfo> = self.create_quadrant_vec(strt);
    let strt_tuple = strt.gen_sect_tuple();
    let tgt_tuple = tgt.gen_sect_tuple();

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
        let sector_info: SectorInfo = find_actual_sector_info(&quad_vec, (x7 as i8, y7 as i8));
        //println!("{} {} {:?}", x7, y7, sector_info.obj_type);
        if sector_info.get_sector_type() != SectorType::Enterprise
            && sector_info.get_sector_type() != SectorType::Empty
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
/// create a quadrant informationm vector from the supplied quadrant vector and enterprise.
pub fn create_bad_guy_qi_vec(
    qi_vec: &Vec<SectorInfo>,
    interest_loc: Location,
    chk_path: bool,
) -> Vec<SectorInfo> {
    let mut n_vec: Vec<SectorInfo> = Vec::new();
    for si in qi_vec.iter() {
        let n_info = *si;
        let bad_guy_type: SectorType = n_info.get_sector_type();
        if bad_guy_type == SectorType::Klingon || bad_guy_type == SectorType::Romulan {
            if chk_path {
                let path_res =
                    is_straight_line_path_clear(&qi_vec, interest_loc, n_info.get_location());
                match path_res {
                    Ok(_) => {
                        if path_res.unwrap() {
                            n_vec.push(n_info);
                            //println!("potential {:?}", n_info);
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
