#![warn(missing_docs)]
//! # statistics.rs
//!
//! This is stuff related to the present state of the game
//!

use crate::manifest::entity::Entity;
use crate::manifest::enums::EntityType;

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
pub fn calculate(qi_vec: &Vec<Entity>) -> SummaryStats {
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
        match n_info.get_sector_type() {
            EntityType::Klingon => stats.num_alive_klingons += 1,
            EntityType::KilledKlingon => {
                stats.num_killed_klingons += 1;
                stats.cur_score += 100;
            }
            EntityType::Romulan => stats.num_alive_romulans += 1,
            EntityType::KilledRomulan => {
                stats.num_killed_romulans += 1;
                stats.cur_score += 20;
            }
            EntityType::Star => stats.num_stars += 1,
            EntityType::Planet => stats.num_planets += 1,
            EntityType::Starbase => stats.num_star_bases += 1,
            EntityType::PlayerShip => stats.num_enterprise += 1,
            _ => {}
        }
    }
    stats
}
