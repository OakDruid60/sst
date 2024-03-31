#![warn(missing_docs)]
//! # statistics.rs
//!
//! This is stuff related to the present state of the game
//!

use crate::manifest::entity::Entity;
use crate::manifest::enums::SectorType;

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
            SectorType::Klingon => stats.num_alive_klingons += 1,
            SectorType::KilledKlingon => {
                stats.num_killed_klingons += 1;
                stats.cur_score += 100;
            }
            SectorType::Romulan => stats.num_alive_romulans += 1,
            SectorType::KilledRomulan => {
                stats.num_killed_romulans += 1;
                stats.cur_score += 20;
            }
            SectorType::Star => stats.num_stars += 1,
            SectorType::Planet => stats.num_planets += 1,
            SectorType::Starbase => stats.num_star_bases += 1,
            SectorType::Enterprise => stats.num_enterprise += 1,
            _ => {}
        }
    }
    stats
}
