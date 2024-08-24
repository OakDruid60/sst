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

use crate::astro::AstroObject;
use crate::ship_info::PlayerShip;
//use crate::manifest::constants::{DEBUG, DEBUG_FILE_NAME, MAX_GALAXY_SIZE_I8, //MAX_SECTOR_SIZE_I8};
//use crate::manifest::entity::Entity;
//use crate::manifest::enums::EntityType;
//use crate::manifest::statistics::SummaryStats;

//use rand::Rng;
use serde::{Deserialize, Serialize};
//use serde_json::from_str;
use std::collections::HashMap;
//use std::fs::File;
//use std::io::{Read, Write};

// ======================================================================
// ======================================================================
/// Manifest
// ======================================================================
// ======================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub cur_star_date: i32,
    pub end_star_date: i32,
    galaxy_map: HashMap<String, AstroObject>,
    pub test_cmds_vec: Vec<String>,
    player_ship: PlayerShip,
    pub password: String,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            //charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            galaxy_map: HashMap::new(),
            test_cmds_vec: Vec::new(),
            player_ship: PlayerShip::new(),
            password: "jap".to_string(),
        }
    }

    pub fn galaxy_map(&self) -> &HashMap<String, AstroObject> {
        &self.galaxy_map
    }
    pub fn player_ship(&self) -> PlayerShip {
        &self.player_ship.clone();
    }
    // =========================================================================
    /// # create_quadrant_vec
    ///
    /// create a quadrant vector of objects for the location of interest.
    ///
    pub fn create_quadrant_vec(&self, interest_loc: AstroObject) -> Vec<AstroObject> {
        let mut n_vec: Vec<AstroObject> = Vec::new();
        for n_info in self.galaxy_map.values() {
            //let n_info = *si;
            if n_info.is_same_quad(&interest_loc) {
                n_vec.push(*n_info);
            }
        }
        n_vec
    }
}
/*
    // ==========================================================================
    /// # game_stat_create_disp_vec
    ///
    pub fn game_stat_create_disp_vec(self, create_complete: bool) -> Vec<String> {
        let input_vec: Vec<Entity>;
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
    let mut n_type_vec: Vec<Entity> = Vec::new();

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
pub fn find_actual_sector_info(orig_vec: &Vec<Entity>, sect: (i8, i8)) -> Entity {
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
    quad_vec: &Vec<Entity>,
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
        if sector_info.get_sector_type() != EntityType::PlayerShip
            && sector_info.get_sector_type() != EntityType::Empty
            && sector_info.get_sector_type() != EntityType::KilledKlingon
            && sector_info.get_sector_type() != EntityType::KilledRomulan
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
    qi_vec: &Vec<Entity>,
    interest_loc: Entity,
    chk_path: bool,
) -> Vec<Entity> {
    let mut n_vec: Vec<Entity> = Vec::new();
    for si in qi_vec.iter() {
        let n_info = *si;
        let bad_guy_type: EntityType = n_info.get_sector_type();
        if bad_guy_type == EntityType::Klingon || bad_guy_type == EntityType::Romulan {
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

*/
