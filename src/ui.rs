#![warn(missing_docs)]
//! # ui.rs
//!
//! This is stuff related to the user interface (ui)

pub mod cmd_proc;
pub mod help_screen;
pub mod logo_screen;
pub mod lrs;
pub mod srs;
pub mod stat_screen;

//use serde_json::to_string;
use colored::Colorize;
/*
use std::io::{stdin, stdout, Write};

pub mod help_screen;
pub mod logo_screen;
pub mod lrs;
pub mod stat_screen;
pub mod srs;
*/
use crate::astro::{AstroObject, AstroType};
use crate::manifest::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::manifest::Manifest;
/*
//use crate::enterprise::ShipInfo;
use crate::manifest::constants::MAX_GALAXY_SIZE_I8;
//use crate::manifest::enums::{CmdType, EntityType};
use crate::manifest::{Manifest, Galaxy};
*/

pub const BORDER_COLOR_RED: &str = "\x1b[91m";
pub const BORDER_COLOR_YELLOW: &str = "\x1b[93m";
pub const BORDER_COLOR_GREEN: &str = "\x1b[92m";
pub const BORDER_COLOR_CYAN: &str = "\x1b[96m";
pub const BORDER_HORZ_60: &str = "════════════════════════════════════════════════════════════";
pub const BORDER_VERT: &str = "║";
pub const BORDER_UR: &str = "╗";
pub const BORDER_UL: &str = "╔";
pub const BORDER_LR: &str = "╝";
pub const BORDER_LL: &str = "╚";
pub const BORDER_MR: &str = "╣";
pub const BORDER_ML: &str = "╠";

pub const COLOR_RESET: &str = "\x1b[0m";

// ==========================================================
/// # disp_title
///
/// New attempt at a command title bar
pub fn disp_title(title: &str, g_info: &Manifest, bc: &str) {
    let tmp_title = format!("{}", title.underline());
    println!();
    println!("  {bc}{BORDER_UL}{BORDER_HORZ_60}{BORDER_UR}{COLOR_RESET}");
    println!(
        "  {bc}{BORDER_VERT}{COLOR_RESET} {: <38}{: >28} {bc}{BORDER_VERT}{COLOR_RESET}",
        tmp_title,
        g_info.player_ship().get_entity().to_compact_string()
    );
    println!("  {bc}{BORDER_ML}{BORDER_HORZ_60}{BORDER_MR}{COLOR_RESET}");
}

// =====================================================================
/// # validate_x_y_input
///
/// make sure the new cmd x y values are valid
pub fn validate_x_y_input(cmd_vector: &[String], max: i8) -> Result<(i8, i8), String> {
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

    Ok((xx, yy))
}

// =====================================================================
/// # alert_status_of_quadrant
///
pub fn alert_status_of_quadrant(qi_vec: &Vec<AstroObject>) -> String {
    let cur_alert = calc_alert_status(qi_vec);
    let mut stat_string = "Normal".normal().to_string();
    if cur_alert == AlertStatus::Docked {
        stat_string = "DOCKED".normal().to_string();
        return stat_string;
    }
    'outer: for si in qi_vec.iter() {
        let n_info = *si;
        if n_info.get_astro_type() == AstroType::Klingon {
            stat_string = "RED   ".to_string();
            break 'outer;
        } else if n_info.get_astro_type() == AstroType::Romulan {
            stat_string = "YELLOW".to_string();
        }
    }
    return stat_string;
}

pub fn alert_status_of_quadrant2(qi_vec: &Vec<AstroObject>) -> &str {
    let cur_alert = calc_alert_status(qi_vec);
    let mut stat_string = BORDER_COLOR_GREEN;
    if cur_alert == AlertStatus::Docked {
        stat_string = BORDER_COLOR_CYAN;
        return stat_string;
    }
    'outer: for si in qi_vec.iter() {
        let n_info = *si;
        if n_info.get_astro_type() == AstroType::Klingon {
            stat_string = BORDER_COLOR_RED;
            break 'outer;
        } else if n_info.get_astro_type() == AstroType::Romulan {
            stat_string = BORDER_COLOR_YELLOW;
        }
    }
    stat_string
}
// ===============================
/// # calc_alert_status
///
/// Calculate some stuff
pub fn calc_alert_status(qi_vec: &Vec<AstroObject>) -> AlertStatus {
    let mut cur_alert = AlertStatus::Normal;
    // find the enterprise
    let enterprise_vec = crate::manifest::create_vec_of_type(qi_vec, AstroType::PlayerShip);
    let star_base_vec = crate::manifest::create_vec_of_type(qi_vec, AstroType::Starbase);
    if !star_base_vec.is_empty() {
        let distance = enterprise_vec[0].calc_sector_distance(star_base_vec[0]);
        if distance < 1.42 {
            cur_alert = AlertStatus::Docked;
        }
    }
    if cur_alert != AlertStatus::Docked {
        'outer: for si in qi_vec.iter() {
            let n_info = *si;
            if n_info.get_astro_type() == AstroType::Klingon {
                cur_alert = AlertStatus::Red;
                break 'outer;
            } else if n_info.get_astro_type() == AstroType::Romulan {
                cur_alert = AlertStatus::Yellow;
            }
        }
    }
    cur_alert
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
