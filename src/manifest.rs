#![warn(missing_docs)]
//! # manifest.rs
//!
//! Contains the GameData structure.  The directory contains the constant,
//! entity, enum, and statistics sourece.
//!

use crate::astro::{AstroObject, AstroType};
use crate::constants::{DEBUG, DEBUG_FILE_NAME, MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::ship_info::PlayerShip;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use colored::*;

// todo create and switch to a branch
// todo  iss8 create macro for heart of contruct galaxy
macro_rules! populate_sector {
    ($gx:expr, $gy:expr,$qx:expr, $qy:expr,  $ast_type:expr, $ast_rng:expr, $g_map:expr) => {
        //let trial_quad_x: i8 = xx;
        //let trial_quad_y: i8 = yy;

        // planets
        for _counter in 0..rand::thread_rng().gen_range($ast_rng) {
            let mut existing_collision: bool = false;
            while !existing_collision {
                let sx: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                let sy: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
                let n_info: AstroObject =
                    AstroObject::create(($gx, $gy, $qx, $qy, sx, sy), $ast_type);

                //                        n_gal_x,
                //                        n_gal_y,
                //                        trial_quad_x,
                //                        trial_quad_y,
                //                        trial_sect_x,
                //                        trial_sect_y,
                //                    ),
                //                    AstroType::Planet,
                //               );
                let n_key = n_info.to_key_string();
                existing_collision = $g_map.contains_key(&n_key);
                if !existing_collision {
                    $g_map.insert(n_key, n_info);
                    break;
                }
            }
        }
    };
}

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
    pub charted: [[bool; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
    pub test_cmds_vec: Vec<String>,
    pub player_ship: PlayerShip,
    pub password: String,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            uni_map: HashMap::new(),
            test_cmds_vec: Vec::new(),
            player_ship: PlayerShip::new(),
            password: "jap".to_string(),
        }
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
        // clang-format off
        let trial_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        let trial_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        populate_sector!(n_gal_x,n_gal_y, trial_quad_x,trial_quad_y, AstroType::PlayerShip, 1..=1, n_galaxy_map);
        // clang-format on
    /*    
    let mut existing_collision = false;
        while !existing_collision {
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
        */
        //for (key, value) in n_galaxy_map.iter() {
        //    println!("{} {:?}", key, value.get_astro_type());
        //}
        //        println!("=============================================");

        // now populate other things into each quadrant
        for xx in 0..MAX_GALAXY_SIZE_I8 {
            for yy in 0..MAX_GALAXY_SIZE_I8 {
                //let trial_quad_x: i8 = xx;
                //let trial_quad_y: i8 = yy;

                // clang-format off
                // planets
                populate_sector!(n_gal_x,n_gal_y, xx,yy, AstroType::Planet, 0..3, n_galaxy_map);
                // Stars
                populate_sector!(n_gal_x,n_gal_y, xx,yy, AstroType::Star, 1..5, n_galaxy_map);
                // Stars
                populate_sector!(n_gal_x,n_gal_y, xx,yy, AstroType::Klingon, 0..2, n_galaxy_map);
                // Stars
                populate_sector!(n_gal_x,n_gal_y, xx,yy, AstroType::Romulan, 0..3, n_galaxy_map);
                // clang-format on

                /*               for _counter in 0..rand::thread_rng().gen_range(0..3) {
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
                */
            }
        }
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
        let input_vec: Vec<AstroObject> = if create_complete {
            self.uni_map.values().cloned().collect()
        } else {
            self.create_quadrant_vec(self.clone().player_ship.get_entity())
        };
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
/// # isolate_cur_quadrant
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

// =========================================================================
/// # isolate_type
/// isolate the type from the universe
///
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

// =========================================================================
/// # create_vec_of_type
///  From a vector of objests
///
pub fn create_vec_of_type(orig_vec: &[AstroObject], n_type: AstroType) -> Vec<AstroObject> {
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
pub fn find_actual_sector_info(orig_vec: &[AstroObject], sect: (i8, i8)) -> AstroObject {
    for si in orig_vec.iter() {
        let n_info = *si;
        if n_info.is_same_sect_tuple(sect) {
            return n_info;
        }
    }

    AstroObject::create((0, 0, 99, 99, 99, 99), AstroType::Empty)
}

// =================================================================
/// # is_straight_line_path_clear
///
pub fn is_straight_line_path_clear(
    quad_vec: &[AstroObject],
    strt: AstroObject,
    tgt: AstroObject,
) -> Result<bool, String> {
    let strt_tuple = strt.ret_sect_tuple();
    let tgt_tuple = tgt.ret_sect_tuple();

    let mut delta_x: f64 = tgt_tuple.0 as f64 - strt_tuple.0 as f64;
    let mut delta_y: f64 = tgt_tuple.1 as f64 - strt_tuple.1 as f64;
    let distance: f64 = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

    delta_x /= distance;
    delta_y /= distance;

    let mut trial_loc_x = strt_tuple.0 as f64 + 0.5;
    let mut trial_loc_y = strt_tuple.1 as f64 + 0.5;

    // check path for clearance

    for _ in 0..(distance as usize) {
        trial_loc_x += delta_x;
        trial_loc_y += delta_y;
        let x7 = (trial_loc_x.floor()) as i32;
        let y7 = (trial_loc_y.floor()) as i32;
        let sector_info: AstroObject = find_actual_sector_info(quad_vec, (x7 as i8, y7 as i8));
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
                ));
            }
        }
    }

    Ok(true)
}

// ==========================================================================
/// # create_bad_guy_qi_vec
///
/// create a quadrant informationm vector from the supplied quadrant vector
/// and location of interest (probably the enterprise).
pub fn create_bad_guy_qi_vec(
    qi_vec: &[AstroObject],
    interest_loc: AstroObject,
    chk_path: bool,
) -> Vec<AstroObject> {
    let mut n_vec: Vec<AstroObject> = Vec::new();
    for si in qi_vec.iter() {
        let n_info = *si;
        let bad_guy_type: AstroType = n_info.get_astro_type();
        if bad_guy_type == AstroType::Klingon || bad_guy_type == AstroType::Romulan {
            if chk_path {
                let path_res = is_straight_line_path_clear(qi_vec, interest_loc, n_info);
                if path_res.is_ok() && path_res.unwrap() {
                    n_vec.push(n_info);
                }
            } else {
                n_vec.push(n_info);
            }
        }
    }

    n_vec
}

// =================================================================
/// # create_qi_enemy_vec
///
///
pub fn create_qi_enemy_vec(
    g_info: &Manifest,
) -> Result<(Vec<AstroObject>, Vec<AstroObject>), String> {
    let qi_vec = g_info.create_quadrant_vec(g_info.player_ship.get_entity());
    let potential_bad_guys =
        crate::manifest::create_bad_guy_qi_vec(&qi_vec, g_info.player_ship.get_entity(), false);
    if potential_bad_guys.is_empty() {
        return Err("No enemy targets found in the current quadrant.".to_string());
    }
    let potential_bad_guys =
        crate::manifest::create_bad_guy_qi_vec(&qi_vec, g_info.player_ship.get_entity(), true);
    if potential_bad_guys.is_empty() {
        return Err("No enemy targets found in current quadrant along a straight line path from the player ship to the target that is not blocked by some other object.".to_string());
    }
    Ok((qi_vec.clone(), potential_bad_guys.clone()))
}

// =================================================================
/// # calc_distance_to_enemy
///
///
pub fn calc_distance_to_enemy(
    g_info: &Manifest,
    potential_bad_guys: Vec<AstroObject>,
    enemy_type: AstroType,
) -> (bool, f64, AstroObject) {
    let mut n_info: AstroObject;
    let mut found_it = false;
    let mut potential_enemies = potential_bad_guys.clone();
    let mut tgt_sector = potential_enemies[0]; //.clone();
    let mut current_distance: f64 = 1000.0;
    for si in potential_enemies.iter_mut() {
        n_info = *si;
        if n_info.get_astro_type() == enemy_type {
            if !found_it {
                found_it = true;
                tgt_sector = n_info; //.clone();
                current_distance = tgt_sector.calc_sector_distance(g_info.player_ship.get_entity());
            } else {
                let n_tgt_sector = n_info; //.clone();
                let new_distance: f64 =
                    n_tgt_sector.calc_sector_distance(g_info.player_ship.get_entity());
                if new_distance < current_distance {
                    current_distance = new_distance;
                    tgt_sector = n_tgt_sector;
                }
            }
        }
    }
    (found_it, current_distance, tgt_sector)
}

/// Thaw a game.
///
/// The .sst file type is as follows:
///
/// (password)0x1e(json data for Universe object)
pub fn thaw(cmd_vector: &[String]) -> Option<Manifest> {
    let mut save_file_name: &str = DEBUG_FILE_NAME;
    if cmd_vector.len() == 2 {
        save_file_name = cmd_vector[1].as_str();
    }
    let mut save_file: File = File::open(save_file_name).expect("Reason");
    //let uni: Manifest;

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
        .map(String::from)
        .collect();

    let uni = match from_str(raw_parts[1].as_str()) {
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
    Some(uni)
}

// ==========================================================================
/// # freeze
///
///
//pub fn freeze (filename: Option<String>, uni: &GameWideData) {
pub fn freeze(uni: &Manifest, cmd_vector: &[String]) {
    /* let filename = match filename {
        Some(v) => v,
        None => input("Filename: ")
    };
    */
    let mut save_file_name = DEBUG_FILE_NAME;
    if cmd_vector.len() == 2 {
        save_file_name = cmd_vector[1].as_str();
    }
    let mut file = match File::create(save_file_name) {
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
pub fn calculate(qi_vec: &[AstroObject]) -> SummaryStats {
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
pub fn compact_summary_string(qi_vec: &[AstroObject]) -> String {
    let stats = crate::manifest::calculate(qi_vec);
    let mut encoded = String::new();

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
    encoded = String::new();
    if stats.num_enterprise > 0 {
        encoded.push_str(format!("<{}>", tmp2).as_str())
    } else {
        encoded.push_str(format!(" {} ", tmp2).as_str())
    }
    encoded.to_string()
}
