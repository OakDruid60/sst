#![warn(missing_docs)]
//! # astro.rs
//!
//!
//! This is stuff related to how space is organised.

//use crate::manifest::constants::MAX_SECTOR_SIZE_I8;
//use crate::manifest::enums::{DamageType, EntityType};
//use crate::enterprise::ShipInfo;
//use crate::manifest::statistics::SummaryStats;

//use rand::Rng;
//use serde::{Deserialize, Serialize};
//use serde_json::from_str;
//use std::fmt;
//use std::fs::File;
//use std::io::{Read, Write};
/*
// =============================
// =============================
/// # SpaceLabel
///
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SpaceLabel {
    label: (i8, i8),
}
impl SpaceLabel {
    pub fn new(a: i8, b: i8) -> Self {
        Self { label: (a, b) }
    }
    pub fn new_random(max: i8) -> Self {
        Self { label:(
            rand::thread_rng().gen_range(0..max),
            rand::thread_rng().gen_range(0..max),
        ) } 
    }
    // =============================
    /// ## is_same_label
    ///
    pub fn is_same_label(self, comp: &SpaceLabel) -> bool {
        if self.label.0 == comp.label.0 && self.label.1 == comp.label.1 {
            return true;
        }
        false
    }
}

// =============================
// =============================
/// # SpaceDesignator
///
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SpaceDesignator {
    designator: (GalaxyLabel, QuadrantLabel, SectorLabel),
}

// =======================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct GalaxyLabel {
    label: SpaceLabel,
}
impl GalaxyLabel {
    pub fn new(a: i8, b: i8) -> Self {
        Self {
            label: SpaceLabel::new(a, b),
        }
    }
}

// =======================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct QuadrantLabel {
    label: SpaceLabel,
}
impl QuadrantLabel {
    pub fn new(a: i8, b: i8) -> Self {
        Self {
            label: SpaceLabel::new(a, b),
        }
    }
}

// =======================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SectorLabel {
    label: SpaceLabel,
}
impl SectorLabel {
    pub fn new(a: i8, b: i8) -> Self {
        Self {
            label: SpaceLabel::new(a, b),
        }
    }
}

//#[derive(Copy, Clone, Serialize, Deserialize)]
//pub struct Sect {
//    loc: (i8, i8),
//}
//impl Sect {
//    pub fn new(x: i8, y: i8) -> Self {
//        Self { loc: (x, y) }
//    }
//}



/*
// ======================================================================
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Entity17 {
    // id: (i8, i8, i8, i8, SectorType),
    q: Quad,
    s: Sect,
    t: SectorType,
}
impl Default for Entity17 {
    fn default() -> Self {
        Self::new()
    }
}
impl Entity17 {
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
//galaxy_vec: Vec<Galaxy>;
*/


// =============================
// =============================
/// # Galaxy
///
pub struct GalaxyOld {
    designator: GalaxyLabel,
    quadrant_vec: Vec<Quadrant>,
    //pub charted: [[bool; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
}

// =============================
// =============================
/// # Quadrant
///
pub struct Quadrant {
    designator: QuadrantLabel,
    sector_vec: Vec<Sector>,
    charted: bool,
}

// =============================
// =============================
/// # Sector
///
pub struct Sector {
    designator: SectorLabel,
    entity: Entity,
}

// =============================
// =============================
/// # Entity
///
pub struct Entity {
    designator: SpaceDesignator,
    typ: EntityType,
    damage_vec: Vec<Damage>,
}
// # entity.rs
//
// This is stuff related to the entire game.

use crate::manifest::constants::MAX_SECTOR_SIZE_I8;
//use crate::manifest::enums::EntityType;
//use crate::enterprise::ShipInfo;
//use crate::manifest::statistics::SummaryStats;

use rand::Rng;
use serde::{Deserialize, Serialize};
//use serde_json::from_str;
use std::fmt;
//use std::fs::File;
//use std::io::{Read, Write};

#[derive( PartialEq, Serialize, Deserialize)]
pub enum EntityType {
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
pub struct Quad {
    loc: (i8, i8),
}
impl Quad {
    pub fn new(x: i8, y: i8) -> Self {
        Self { loc: (x, y) }
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
    t: EntityType,
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
            t: EntityType::Empty,
        }
    }

    // =============================
    /// # create
    ///
    pub fn create(n: (i8, i8, i8, i8, EntityType)) -> Self {
        Self {
            //    id: (n.0, n.1, n.2, n.3, n.4),
            q: Quad::new(n.0, n.1),
            s: Sect::new(n.2, n.3),
            t: n.4,
        }
    }

    // =============================
    //
    pub fn get_sector_type(self) -> EntityType {
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
        let mut tmp_s = EntityType::Empty;
        match tmp_t {
            EntityType::Klingon => {
                tmp_s = EntityType::KilledKlingon;
            }
            EntityType::Romulan => {
                tmp_s = EntityType::KilledRomulan;
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
    pub fn create_new_random_player_ship_in_this_quad(nq: (i8, i8)) -> Self {
        Self {
            q: Quad::new(nq.0, nq.1),
            s: Sect::new(
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
                rand::thread_rng().gen_range(0..MAX_SECTOR_SIZE_I8),
            ),
            t: EntityType::PlayerShip,
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
// =====================================================================
/// #CmdType
///  The type of commands
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmdType {
    Help,
    Quit,
    SRS,
    LRS,
    Phaser,
    Torpedoe,
    Move,
    Jump,
    Test,
    Status,
    Restore,
    Save,
    RecordOn,
    RecordOff,
    Empty,
}

// =====================================================================
/// #AlertStatus
///  The alert status types
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlertStatus {
    Normal,
    Docked,
    Red,
    Yellow,
}

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
    
// ======================================================================
// ======================================================================
/// Galaxy
// ======================================================================
// ======================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Galaxy {
    pub designator: GalaxyLabel,
    pub data_vec: Vec<Entity>,
}

impl Galaxy {
    pub fn new() -> Self {
        Self {
            cur_star_date: 0,
            end_star_date: 0,
            //charted: [[false; MAX_GALAXY_SIZE_I8 as usize]; MAX_GALAXY_SIZE_I8 as usize],
            galaxy_vec: Vec::new(),
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
    pub fn create_quadrant_vec(&self, interest_loc: Entity) -> Vec<Entity> {
        let mut n_vec: Vec<Entity> = Vec::new();
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
pub fn create_vec_of_type(orig_vec: &Vec<Entity>, n_type: EntityType) -> Vec<Entity> {
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
        Entity::create((n_quad_x, n_quad_y, n_sect_x, n_sect_y, EntityType::Starbase));
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
            EntityType::PlayerShip,
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
                        EntityType::Planet,
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
                        EntityType::Star,
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
                        EntityType::Klingon,
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
                        EntityType::Romulan,
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