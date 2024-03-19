#![warn(missing_docs)]
//! # helpers.rs
//!
//! This supplies various helper functions used throughout the game.
//!
use crate::manifest::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::manifest::entity::Entity;
use crate::manifest::enums::{AlertStatus, SectorType};

use colored::*;

// ==============================================================
/// # compact_summary_string
///
///  designed to work at the quadrant info level, not galaxy level
pub fn compact_summary_string(qi_vec: &Vec<Entity>) -> String {
    let stats = crate::manifest::statistics::calculate(qi_vec);
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

// =============================
/// # alert_status_of_quadrant
/// Return the alert status for all sectors in the quadrant.
///
pub fn alert_status_of_quadrant(qi_vec: &Vec<Entity>) -> String {
    let cur_alert = calc_alert_status(qi_vec);
    let mut stat_string = "Normal".normal().to_string();
    if cur_alert == AlertStatus::Docked {
        stat_string = "DOCKED".normal().to_string();
        return stat_string;
    }
    'outer: for si in qi_vec.iter() {
        let n_info = *si;
        if n_info.get_sector_type() == SectorType::Klingon {
            stat_string = "RED   ".to_string();
            break 'outer;
        } else if n_info.get_sector_type() == SectorType::Romulan {
            stat_string = "YELLOW".to_string();
        }
    }
    return stat_string;
}

// ===============================
/// # calc_alert_status
///
/// Calculate some stuff
pub fn calc_alert_status(qi_vec: &Vec<Entity>) -> AlertStatus {
    let mut cur_alert = AlertStatus::Normal;
    // find the enterprise
    let enterprise_vec = crate::manifest::create_vec_of_type(qi_vec, SectorType::Enterprise);
    let star_base_vec = crate::manifest::create_vec_of_type(qi_vec, SectorType::Starbase);
    if star_base_vec.len() != 0 {
        let distance = enterprise_vec[0].calc_sector_distance(star_base_vec[0]);
        if distance < 1.42 {
            cur_alert = AlertStatus::Docked;
        }
    }
    if cur_alert != AlertStatus::Docked {
        'outer: for si in qi_vec.iter() {
            let n_info = *si;
            if n_info.get_sector_type() == SectorType::Klingon {
                cur_alert = AlertStatus::Red;
                break 'outer;
            } else if n_info.get_sector_type() == SectorType::Romulan {
                cur_alert = AlertStatus::Yellow;
            }
        }
    }
    cur_alert
}

// =====================================================================
/// # validate_x_y_input
///
/// make sure the new cmd x y values are valid
pub fn validate_x_y_input(cmd_vector: &Vec<String>, max: i8) -> Result<(i8, i8), String> {
    let mut chk_max = MAX_GALAXY_SIZE_I8;
    let mut chk_title = "Quadrant";
    if max == MAX_SECTOR_SIZE_I8 {
        chk_max = MAX_SECTOR_SIZE_I8;
        chk_title = "Sector";
    }

    chk_max -= 1;
    // handle the x value
    let res_x: Result<i8, core::num::ParseIntError> = cmd_vector[1].trim().parse();
    match res_x {
        Ok(_) => {}
        Err(_e) => {
            let err_string = format!(
                "Please type a number for the new X coord of the {}!",
                chk_title
            )
            .to_string();
            return Err(err_string);
        }
    }
    let xx: i8 = res_x.unwrap();
    if xx > chk_max {
        let err_string = format!(
            "The x value {} is outside the range of 0..{} for a {}.",
            xx, chk_max, chk_title
        )
        .to_string();
        return Err(err_string);
    }

    // Now handle y value
    let res_y: Result<i8, core::num::ParseIntError> = cmd_vector[2].trim().parse();
    match res_y {
        Ok(_) => {}
        Err(_e) => {
            let err_string = format!(
                "Please type a number for the new Y coord of the {}!",
                chk_title
            )
            .to_string();
            return Err(err_string);
        }
    }
    let yy: i8 = res_y.unwrap();
    if yy > chk_max {
        let err_string = format!(
            "The y value {} is outside the range of 0..{} for a {}.",
            yy, chk_max, chk_title
        )
        .to_string();
        return Err(err_string);
    }

    return Ok((xx as i8, yy as i8));
}
