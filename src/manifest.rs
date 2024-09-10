#![warn(missing_docs)]
//! # manifest.rs
//!
//! Contains the GameData structure.  The directory contains the constant,
//! entity, enum, and statistics sourece.
//!

pub mod constants; // various constants like the size of the galaxy
                   //pub mod entity;
                   //pub mod enums;
                   //pub mod galaxy;
                   //pub mod spaceorg;
pub mod statistics;

use crate::astro::{AstroObject, AstroType};
use crate::manifest::constants::{DEBUG, DEBUG_FILE_NAME, MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::ship_info::PlayerShip; //,;
                                  //use crate::manifest::entity::Entity;
                                  //use crate::manifest::enums::AstroType;
                                  //use crate::manifest::statistics::SummaryStats;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use colored::*;

// ======================================================================
// ======================================================================
/// Manifest
// ======================================================================
// ======================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub cur_star_date: i32,
    pub end_star_date: i32,
    pub uni_map: HashMap<String, AstroObject>,
    pub test_cmds_vec: Vec<String>,
    pub player_ship: PlayerShip,
    pub password: String,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            //charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            uni_map: HashMap::new(),
            test_cmds_vec: Vec::new(),
            player_ship: PlayerShip::new(),
            password: "jap".to_string(),
        }
    }

    pub fn uni_map(&self) -> &HashMap<String, AstroObject> {
        &self.uni_map
    }
    pub fn player_ship(&self) -> &PlayerShip {
        &self.player_ship
    }

    // =========================================================================
    /// # construct_galaxy
    ///
    pub fn construct_galaxy(gal_coord: (i8, i8)) -> HashMap<String, AstroObject> {
        let mut n_galaxy_map: HashMap<String, AstroObject> = HashMap::with_capacity(400);
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
        for (key, value) in n_galaxy_map.iter() {
            println!("{} {:?}", key, value.get_astro_type());
        }
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

        //for (key, value) in n_galaxy_map.iter() {
        //let n_info = *si;
        //    println!("{} {:?}", key, value.get_astro_type());
        //}
        //n_galaxy_map.shrink_to(n_galaxy_map.len()+50);
        n_galaxy_map
    }
    // =========================================================================
    /// # create_quadrant_vec
    ///
    /// create a quadrant vector of objects for the location of interest.
    ///
    pub fn create_quadrant_vec(&self, interest_loc: AstroObject) -> Vec<AstroObject> {
        let mut n_vec: Vec<AstroObject> = Vec::new();
        for n_info in self.uni_map.values() {
            //let n_info = *si;
            if n_info.is_same_quad(&interest_loc) {
                n_vec.push(*n_info);
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
            input_vec = self.uni_map.values().cloned().collect();
        } else {
            input_vec = self.create_quadrant_vec(self.clone().player_ship.get_entity());
        }
        let summary: SummaryStats = crate::manifest::calculate(&input_vec);

        let mut human_out: Vec<String> = Vec::new();
        human_out.push(format!(
            "  Enterprise ┃ energy:{:<6} torp:{:<3}",
            self.player_ship.get_energy(),
            self.player_ship.get_torpedoes()
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
pub fn isolate_cur_quadrant(g_info: &Manifest) -> Vec<AstroObject> {
    let mut n_type_vec: Vec<AstroObject> = Vec::new();
    let comp = &g_info.player_ship.get_entity();
    for ao in g_info.uni_map.values() {
        //let n_info = *si;
        if ao.is_same_quad(comp) {
            n_type_vec.push(*ao);
        }
    }

    n_type_vec
}

pub fn isolate_quadrant(g_info: &Manifest, comp: &AstroObject) -> Vec<AstroObject> {
    let mut n_type_vec: Vec<AstroObject> = Vec::new();

    for ao in g_info.uni_map.values() {
        //let n_info = *si;
        if ao.is_same_quad(comp) {
            n_type_vec.push(*ao);
        }
    }

    n_type_vec
}

pub fn isolate_type(g_info: &Manifest, n_type: AstroType) -> Vec<AstroObject> {
    let mut n_type_vec: Vec<AstroObject> = Vec::new();

    for ao in g_info.uni_map.values() {
        //let n_info = *si;
        if ao.get_astro_type() == n_type {
            n_type_vec.push(*ao);
        }
    }
    n_type_vec
}
pub fn create_vec_of_type(orig_vec: &Vec<AstroObject>, n_type: AstroType) -> Vec<AstroObject> {
    let mut n_type_vec: Vec<AstroObject> = Vec::new();

    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.get_astro_type() == n_type {
            n_type_vec.push(n_info);
        }
    }

    n_type_vec
}

// ============================================================
/// # find_actual_sector_info
/// Given a vector of SectorInfo, return the given sector, or Empty if not found.
pub fn find_actual_sector_info(orig_vec: &Vec<AstroObject>, sect: (i8, i8)) -> AstroObject {
    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.is_same_sect_tuple(sect) {
            return n_info;
        }
    }

    AstroObject::create((0, 0, 99, 99, 99, 99), AstroType::Empty)
} // =================================================================
/// # is_straight_line_path_clear
///
pub fn is_straight_line_path_clear(
    quad_vec: &Vec<AstroObject>,
    strt: AstroObject,
    tgt: AstroObject,
) -> Result<bool, String> {
    let strt_tuple = strt.ret_sect_tuple();
    let tgt_tuple = tgt.ret_sect_tuple();

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
        let sector_info: AstroObject = find_actual_sector_info(&quad_vec, (x7 as i8, y7 as i8));
        //println!("{} {} {:?}", x7, y7, sector_info.obj_type);
        if sector_info.get_astro_type() != AstroType::PlayerShip
            && sector_info.get_astro_type() != AstroType::Empty
            && sector_info.get_astro_type() != AstroType::KilledKlingon
            && sector_info.get_astro_type() != AstroType::KilledRomulan
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
    interest_loc: AstroObject,
    chk_path: bool,
) -> Vec<AstroObject> {
    let mut n_vec: Vec<AstroObject> = Vec::new();
    for si in qi_vec.iter() {
        let n_info = *si;
        let bad_guy_type: AstroType = n_info.get_astro_type();
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

/// Thaw a game.
///
/// The .sst file type is as follows:
///
/// (password)0x1e(json data for Universe object)
pub fn thaw(cmd_vector: &Vec<String>) -> Option<Manifest> {
    let mut save_file_name: &str = DEBUG_FILE_NAME;
    if cmd_vector.len() == 2 {
        save_file_name = cmd_vector[1].as_str();
    }
    let mut save_file: File = File::open(save_file_name).expect("Reason");
    let uni: Manifest;

    let temp = File::open(save_file_name);
    match temp {
        Ok(p) => save_file = p,
        Err(_) => {
            println!("ERROR: Unable to find save file.\n");
        }
    }

    //let pass = input("Password: ");
    let mut enc_data = String::new();
    match save_file.read_to_string(&mut enc_data) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("\nERROR: The save file is corrupted.");
            return None;
        }
    }

    let raw_parts: Vec<String> = enc_data
        .split("\0")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|element| String::from(element))
        .collect();

    uni = match from_str(raw_parts[1].as_str()) {
        Ok(data) => {
            //println!("Restored");
            println!("Game back-up restored from {}", save_file_name);
            data
        }
        Err(_) => {
            println!("\nERROR: The save file is corrupted.");
            return None;
        }
    };
    return Some(uni);
}

// ==========================================================================
/// # freeze
///
///
//pub fn freeze (filename: Option<String>, uni: &GameWideData) {
pub fn freeze(uni: &Manifest, cmd_vector: &Vec<String>) {
    /* let filename = match filename {
        Some(v) => v,
        None => input("Filename: ")
    };
    */
    let mut save_file_name = DEBUG_FILE_NAME;
    if cmd_vector.len() == 2 {
        save_file_name = cmd_vector[1].as_str();
    }
    let mut file = match File::create(&save_file_name) {
        Ok(f) => f,
        Err(e) => {
            if DEBUG {
                println!("{}", e);
            }
            println!("Alas, it is impossible to create a file in that location.");
            return;
        }
    };

    // println!("{}",serde_json::to_string(uni).unwrap().as_str());
    match file.write_all(
        (uni.password.clone() + "\0" + serde_json::to_string(uni).unwrap().as_str()).as_bytes(),
    ) {
        Ok(_) => {}
        Err(_) => println!("I'm sorry, but that file cannot be written to."),
    }

    println!("Game back-up created in {}", save_file_name);
}
// =====================================================================
/// #SummaryStats
///  Information that is the summary for tha supplied vector of SectorInfo
///

#[derive(Copy, Clone, Debug)]
pub struct SummaryStats {
    pub num_alive_klingons: isize,
    pub num_killed_klingons: isize,
    pub num_alive_romulans: isize,
    pub num_killed_romulans: isize,
    pub num_stars: isize,
    pub num_planets: isize,
    pub num_star_bases: isize,
    pub num_enterprise: isize,
    pub cur_score: isize,
}

// ============================================================================
//
impl SummaryStats {}
// =============================
/// # calculate
/// Given an input quad info vaector, calculate the current state of the game
pub fn calculate(qi_vec: &Vec<AstroObject>) -> SummaryStats {
    let mut stats = SummaryStats {
        num_enterprise: 0,
        num_killed_klingons: 0,
        num_killed_romulans: 0,
        num_alive_klingons: 0,
        num_planets: 0,
        num_alive_romulans: 0,
        num_star_bases: 0,
        num_stars: 0,
        cur_score: 0,
    };
    for si in qi_vec.iter() {
        let n_info = *si;
        match n_info.get_astro_type() {
            AstroType::Klingon => stats.num_alive_klingons += 1,
            AstroType::KilledKlingon => {
                stats.num_killed_klingons += 1;
                stats.cur_score += 100;
            }
            AstroType::Romulan => stats.num_alive_romulans += 1,
            AstroType::KilledRomulan => {
                stats.num_killed_romulans += 1;
                stats.cur_score += 20;
            }
            AstroType::Star => stats.num_stars += 1,
            AstroType::Planet => stats.num_planets += 1,
            AstroType::Starbase => stats.num_star_bases += 1,
            AstroType::PlayerShip => stats.num_enterprise += 1,
            _ => {}
        }
    }
    stats
}

// ==============================================================
/// # compact_summary_string
///
///  designed to work at the quadrant info level, not galaxy level
pub fn compact_summary_string(qi_vec: &Vec<AstroObject>) -> String {
    let stats = crate::manifest::calculate(qi_vec);
    let mut encoded = format!("");

    let mut tmp: String = format!("{}", stats.num_alive_klingons);
    if stats.num_alive_klingons > 0 {
        tmp = tmp.red().to_string();
    }
    encoded.push_str(tmp.as_str());

    tmp = format!("{}", stats.num_alive_romulans);
    if stats.num_alive_romulans > 0 {
        tmp = tmp.bright_yellow().to_string();
    }
    encoded.push_str(tmp.as_str());

    tmp = format!("{}", stats.num_star_bases);
    if stats.num_star_bases > 0 {
        tmp = tmp.bright_cyan().to_string();
    }
    encoded.push_str(tmp.as_str());

    tmp = format!("{}", stats.num_planets);
    if stats.num_planets > 0 {
        tmp = tmp.bright_blue().to_string();
    }
    encoded.push_str(tmp.as_str());

    tmp = format!("{}", stats.num_stars);
    encoded.push_str(tmp.as_str());

    // Indicate that the quad contains the enterprise
    let tmp2 = encoded.to_string();
    encoded = format!("");
    if stats.num_enterprise > 0 {
        encoded.push_str(format!("<{}>", tmp2).as_str())
    } else {
        encoded.push_str(format!(" {} ", tmp2).as_str())
    }
    encoded.to_string()
}
