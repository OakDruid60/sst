#![warn(missing_docs)]
//! # disp.rs
//!
//! This is stuff related to displaying information, the sensor scans, the logo, etc.
//!
use crate::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::enums::SectorType;
use crate::gamedata::GameWideData;
use crate::location::Location;
use crate::sector::SectorInfo;

use colored::*;

// ==========================================================================
/// # game_logo
///
/// put the opening game identification info on the console
///
pub fn game_logo() {
    println!("");
    println!(
        "{} v{} -- {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    println!("");

    println!("                     ,------*------,");
    println!("     ,-------------   '---  ------'");
    println!("      '-------- --'      / /");
    println!("          ,---' '-------/ /--,");
    println!("           '----------------'");
    println!("");
    println!(
        "{} {} {} {} \n",
        "     ",
        "USS ENTERPRISE".green(),
        " --- ".clear(),
        "NCC-1701".bright_purple()
    );
}

// ==========================================================================
/// # game_stat_disp
///
pub fn game_stat_disp(g_info: &GameWideData) {
    let human_output: Vec<String> = g_info.clone().game_stat_create_disp_vec(true);

    crate::ui::disp_title("Game Stats");
    for element in &human_output {
        println!("{}", element);
    }
}

// ==========================================================================
/// # short_range_sensor_disp
///
pub fn short_range_sensor_disp(g_info: &GameWideData) {
    let loc_tmp = g_info.enterprise.get_location().clone();
    let qi_vec = g_info.create_quadrant_vec(loc_tmp);
    crate::ui::disp_title("Short Range Scan");

    let g_tmp = g_info.clone();
    let mut human_out: Vec<String> = g_tmp.game_stat_create_disp_vec(false);

    for yy in { 00..MAX_SECTOR_SIZE_I8 }.rev() {
        let mut row_string = String::from("");
        row_string.push_str(format!("{} |", yy).as_str());

        for xx in 0..MAX_SECTOR_SIZE_I8 {
            let cur_sector_info: SectorInfo =
                crate::gamedata::find_actual_sector_info(&qi_vec, (xx, yy));
            let cur_sector_type: SectorType = cur_sector_info.get_sector_type();
            match cur_sector_type {
                SectorType::Empty => row_string.push_str(format!(" {}", ".").as_str()),
                SectorType::Enterprise => {
                    row_string.push_str(format!(" {}", "E".bold().green()).as_str())
                }
                SectorType::Klingon => row_string.push_str(format!(" {}", "k".on_red()).as_str()),
                SectorType::Star => row_string.push_str(format!(" *").as_str()),
                SectorType::Romulan => {
                    row_string.push_str(format!(" {}", "r".on_yellow()).as_str())
                }
                SectorType::Planet => row_string.push_str(format!(" {}", "p".on_blue()).as_str()),
                SectorType::Starbase => row_string.push_str(format!(" {}", "B".cyan()).as_str()),
                SectorType::KilledKlingon => row_string.push_str(format!(" {}", "k").as_str()),
                SectorType::KilledRomulan => row_string.push_str(format!(" {}", "r").as_str()),
            }
        }
        if yy == 9 {
            row_string.push_str(format!("{:>14} |  ", "status").as_str());
            row_string.push_str(crate::helpers::alert_status_of_quadrant(&qi_vec).as_str());
        } else if yy == 8 {
            //row_string.push_str(format!("\n").as_str());
        } else if yy == 7 {
            row_string.push_str(format!("{}", human_out.remove(0)).as_str());
        } else if yy == 5 {
            row_string.push_str(format!("{}", human_out.remove(0)).as_str());
        } else if yy == 4 {
            row_string.push_str(format!("{}", human_out.remove(0)).as_str());
        } else if yy == 3 {
            row_string.push_str(format!("{}", human_out.remove(0)).as_str());
        } else if yy == 2 {
        }
        println!("{}", row_string);
    }
    //
    let row_string = String::from("  |---------------------- ");
    println!("{}", row_string);
    //
    // put in axis points
    let mut row_string = String::from("");
    row_string.push_str(format!("    0 1 2 3 4 5 6 7 8 9  ").as_str());
    // row_string.push_str(format!("{:>20} ", " ").as_str());

    println!("{}", row_string);
}

// ==========================================================================
/// # long_range_sensor_disp
///
/// Put the Long Range sensor scan on the console.  Checks each quadrant
/// to see if it has been charted.  The information is encoded in a 5 character
/// string that is colorized for emphasis.
///
/// **Note** The sectors in the vicinity of the ENTERPRISE are marked as
/// charted even if they have not been.   Also the starbase quadrant is
/// also marked as charted.
///
pub fn long_range_sensor_disp(g_info: &GameWideData) {
    //
    crate::ui::disp_title("Long Range Scan");
    for yy in { 00..MAX_GALAXY_SIZE_I8 }.rev() {
        let mut row_string = format!("{} |", yy);
        for xx in 0..MAX_GALAXY_SIZE_I8 {
            let tmp_xx: i8 = xx as i8;
            let tmp_yy: i8 = yy as i8;
            let zero: i8 = 0 as i8;
            let tmp_loc = Location::create((tmp_xx, tmp_yy, zero, zero));

            let tmp_qi_vec = g_info.create_quadrant_vec(tmp_loc);

            //    let tmp_visited: bool = tmp_quad_info.visited_flag();

            let mut tmp: String = crate::helpers::compact_summary_string(&tmp_qi_vec);
            if g_info.charted[xx as usize][yy as usize] {
                tmp = format!("{}", tmp);
            } else {
                tmp = format!("   .   ");
            }
            row_string.push_str(tmp.as_str());
        }
        if yy == 0 {
            row_string.push_str(
                format!("\n  |--------------------------------------------------------").as_str(),
            );
            row_string.push_str(
                format!(
                    "\n   {:^7}{:^7}{:^7}{:^7}{:^7}{:^7}{:^7}{:^7}\n",
                    0, 1, 2, 3, 4, 5, 6, 7
                )
                .as_str(),
            );
        } else {
            row_string.push_str(format!("\n").as_str());
        }

        print!("{}", row_string);
    }
}

// ==========================================================================
/// # help_screen
///
/// Put the game instructions on the console.
///
/// **Note** needs more details.
///
pub fn help_screen() {
    crate::ui::disp_title("Command Help");

    println!("qui - quit");
    println!("");
    println!("lrs - long range sensor scan");
    println!("srs - short range sensor scan");
    //println!("");
    println!("pha - fire phasers");
    println!("tor - fire torpedoe");
    //println!("");
    println!("jum - jump to new quadrant");
    println!("mov - move to new sector in current quadrant");
    println!("");
    println!("hel - help");
}
