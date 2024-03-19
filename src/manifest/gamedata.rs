#![warn(missing_docs)]
//! # gamedata.rs
//!
//! This is stuff related to the entire game.

use crate::constants::{DEBUG, MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::enums::SectorType;
use crate::ship::ShipInfo;
use crate::statistics::SummaryStats;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};

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
    pub gal_vec: Vec<Entity>,
    pub test_cmds_vec: Vec<String>,
    pub enterprise: ShipInfo,
    pub password: String,
}

impl GameWideData {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            gal_vec: Vec::new(),
            test_cmds_vec: Vec::new(),
            enterprise: ShipInfo::new(),
            password: "jap".to_string(),
        }
    }

    // =========================================================================
    /// # create_quadrant_vec
    ///
    /// create a quadrant information vector for location of interest
    /// of interest.
    pub fn create_quadrant_vec(&self, interest_loc: Entity) -> Vec<Entity> {
        let mut n_vec: Vec<Entity> = Vec::new();
        for si in self.gal_vec.iter() {
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
        let input_vec: Vec<Entity>;
        if create_complete {
            input_vec = self.gal_vec;
        } else {
            input_vec = self.create_quadrant_vec(self.clone().enterprise.get_entity());
        }
        let summary: SummaryStats = crate::statistics::calculate(&input_vec);

        let mut human_out: Vec<String> = Vec::new();
        human_out.push(format!(
            "  Enterprise ┃  energy:{:<6}  torp:{:<4}┃",
            self.enterprise.get_energy(),
            self.enterprise.get_torpedoes()
        ));
        human_out.push(format!(
            " Astro Count ┃  stars:{:<3}   planets:{:<4}┃",
            summary.num_stars, summary.num_planets
        ));
        human_out.push(format!(
            "     Klingon ┃  t:{:<4}   k:{:<4}   a:{:<4}┃",
            summary.num_killed_klingons + summary.num_alive_klingons,
            summary.num_killed_klingons,
            summary.num_alive_klingons
        ));
        human_out.push(format!(
            "     Romulan ┃  t:{:<4}   k:{:<4}   a:{:<4}┃",
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
pub fn create_vec_of_type(orig_vec: &Vec<Entity>, n_type: SectorType) -> Vec<Entity> {
    let mut n_type_vec: Vec<Entity> = Vec::new();

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
pub fn construct_galaxy() -> Vec<Entity> {
    let mut n_galaxy_vec: Vec<Entity> = Vec::new();

    //
    // set initial starbase
    let n_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
    let n_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);

    let n_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
    let n_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);

    let starbase_info: Entity =
        Entity::create((n_quad_x, n_quad_y, n_sect_x, n_sect_y, SectorType::Starbase));
    n_galaxy_vec.push(starbase_info);

    //
    // set initial enterprise
    let mut existing_collision = false;
    while !existing_collision {
        let trial_quad_x: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        let trial_quad_y: i8 = rand::thread_rng().gen_range(0..MAX_GALAXY_SIZE_I8);
        let trial_sect_x: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);
        let trial_sect_y: i8 = rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8);

        let enterprise_info: Entity = Entity::create((
            trial_quad_x,
            trial_quad_y,
            trial_sect_x,
            trial_sect_y,
            SectorType::Enterprise,
        ));
        for si in n_galaxy_vec.iter() {
            if enterprise_info.is_same(si) {
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
                    let n_info: Entity = Entity::create((
                        trial_quad_x,
                        trial_quad_y,
                        trial_sect_x,
                        trial_sect_y,
                        SectorType::Planet,
                    ));
                    for si in n_galaxy_vec.iter() {
                        if n_info.is_same(si) {
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
                    let n_info: Entity = Entity::create((
                        trial_quad_x,
                        trial_quad_y,
                        trial_sect_x,
                        trial_sect_y,
                        SectorType::Star,
                    ));
                    for si in n_galaxy_vec.iter() {
                        if n_info.is_same(si) {
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
                    let n_info: Entity = Entity::create((
                        trial_quad_x,
                        trial_quad_y,
                        trial_sect_x,
                        trial_sect_y,
                        SectorType::Klingon,
                    ));
                    for si in n_galaxy_vec.iter() {
                        if n_info.is_same(si) {
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
                    let n_info: Entity = Entity::create((
                        trial_quad_x,
                        trial_quad_y,
                        trial_sect_x,
                        trial_sect_y,
                        SectorType::Romulan,
                    ));
                    for si in n_galaxy_vec.iter() {
                        if n_info.is_same(si) {
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
    qi_vec: &Vec<Entity>,
    interest_loc: Entity,
    chk_path: bool,
) -> Vec<Entity> {
    let mut n_vec: Vec<Entity> = Vec::new();
    for si in qi_vec.iter() {
        let n_info = *si;
        let bad_guy_type: SectorType = n_info.get_sector_type();
        if bad_guy_type == SectorType::Klingon || bad_guy_type == SectorType::Romulan {
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

pub fn thaw() -> Option<GameWideData> {
    //! Thaw a game.
    //!
    //! The .sst file type is as follows:
    //!
    //! (password)0x1e(json data for Universe object)
    let mut save_file: File = File::open(crate::constants::DEBUG_FILE_NAME).expect("Reason");
    let uni: GameWideData;

    let temp = File::open(crate::constants::DEBUG_FILE_NAME);
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
        Ok(data) => data,
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
pub fn freeze(uni: &GameWideData) {
    /* let filename = match filename {
        Some(v) => v,
        None => input("Filename: ")
    };
    */
    let filename = crate::constants::DEBUG_FILE_NAME; //"tjap.sst"; //.to_string();
    let mut file = match File::create(&filename) {
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

    println!("Game back-up created in {}", filename);
}

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
