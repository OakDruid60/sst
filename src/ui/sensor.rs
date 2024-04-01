#![warn(missing_docs)]
//! # sensor.rs
//!
//! This is stuff related to displaying the sensor scans.

use crate::manifest::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::manifest::entity::Entity;
use crate::manifest::enums::SectorType;
use crate::manifest::Manifest;

use colored::*;

// ==========================================================================
/// # short_range_sensor_disp
///
pub fn short_range_sensor_disp(g_info: &Manifest) {
    let loc_tmp = g_info.enterprise.get_entity().clone();
    let qi_vec = g_info.create_quadrant_vec(loc_tmp);
    crate::ui::disp_title("Short Range Scan", g_info);

    let g_tmp = g_info.clone();
    let mut human_out: Vec<String> = g_tmp.game_stat_create_disp_vec(false);
    for yy in { 00..MAX_SECTOR_SIZE_I8 }.rev() {
        let mut row_string = String::from("");
        row_string.push_str(format!("{} ┃", yy).as_str());

        for xx in 0..MAX_SECTOR_SIZE_I8 {
            let cur_sector_info: Entity =
                crate::manifest::find_actual_sector_info(&qi_vec, (xx, yy));
            let cur_sector_type: SectorType = cur_sector_info.get_sector_type();
            match cur_sector_type {
                SectorType::Empty => row_string.push_str(format!(" {}", ".").as_str()),
                SectorType::Enterprise => {
                print!("E");
                    row_string.push_str(format!(" {}", "E".bold().green()).as_str())
                }
                SectorType::Klingon => {
                    row_string.push_str(format!(" {}", "k".black().on_red()).as_str())
                }
                SectorType::Star => row_string.push_str(format!(" *").as_str()),
                SectorType::Romulan => {
                    row_string.push_str(format!(" {}", "r".black().on_yellow()).as_str())
                }
                SectorType::Planet => row_string.push_str(format!(" {}", "p".on_blue()).as_str()),
                SectorType::Starbase => row_string.push_str(format!(" {}", "B".cyan()).as_str()),
                SectorType::KilledKlingon => row_string.push_str(format!(" {}", "k").as_str()),
                SectorType::KilledRomulan => {
                print!("r");
                row_string.push_str(format!(" {}", "r").as_str())
                }
            }
        }
        row_string.push_str(" ┃ ");

        match yy {
            9 => {
                row_string.push_str(format!("{:>12} ┃ ", "Alert").as_str());
                row_string.push_str(
                    format!(
                        " {:<23} ┃  ",
                        crate::helpers::alert_status_of_quadrant(&qi_vec)
                    )
                    .as_str(),
                );
            }
            7 | 5 | 4 | 3 => {
                row_string.push_str(format!("{}", human_out.remove(0)).as_str());
            }
            _ => {
                row_string.push_str(format!("{:>12} ┃  {:>23} ┃", " ", " ").as_str());
            }
        }
        println!("{}", row_string);
    }
    //
    println!("  ┗{:━^63}┛", "━");
    //
    // put in axis points
    let mut row_string = String::from("");
    row_string.push_str(format!("    0 1 2 3 4 5 6 7 8 9  ").as_str());
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
pub fn long_range_sensor_disp(g_info: &Manifest) {
    //
    crate::ui::disp_title("Long Range Scan", g_info);

    for yy in { 00..MAX_GALAXY_SIZE_I8 }.rev() {
        let mut row_string = format!("{} ┃", yy);
        for xx in 0..MAX_GALAXY_SIZE_I8 {
            let tmp_xx: i8 = xx as i8;
            let tmp_yy: i8 = yy as i8;
            let zero: i8 = 0 as i8;
            let tmp_loc = Entity::create((tmp_xx, tmp_yy, zero, zero, SectorType::Empty));

            let tmp_qi_vec = g_info.create_quadrant_vec(tmp_loc);
            let mut tmp: String = crate::helpers::compact_summary_string(&tmp_qi_vec);
            if g_info.charted[xx as usize][yy as usize] {
                tmp = format!("{}", tmp);
            } else {
                tmp = format!("   .   ");
            }
            row_string.push_str(tmp.as_str());
        }
        row_string.push_str("       ┃");
        if yy == 0 {
            row_string.push_str(
                format!("\n  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  ")
                    .as_str(),
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
