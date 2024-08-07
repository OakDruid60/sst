#![warn(missing_docs)]
//! # srs.rs
//!
//! This is stuff related to displaying the short range sensor scan.

use crate::manifest::constants::MAX_SECTOR_SIZE_I8;
use crate::manifest::entity::Entity;
use crate::manifest::enums::EntityType;
use crate::manifest::Manifest;
use crate::ui::{BORDER_HORZ_60, BORDER_LL, BORDER_LR, BORDER_VERT, COLOR_RESET};

use colored::*;

// ==========================================================================
/// # short_range_sensor_disp
///
pub fn short_range_sensor_disp(g_info: &Manifest) {
    let loc_tmp = g_info.enterprise.get_entity().clone();
    let qi_vec = g_info.create_quadrant_vec(loc_tmp);
    let bc = crate::helpers::alert_status_of_quadrant2(&qi_vec);
    crate::ui::disp_title("Short Range Scan", g_info, bc);

    let g_tmp = g_info.clone();
    let mut human_out: Vec<String> = g_tmp.game_stat_create_disp_vec(false);

    for yy in { 00..MAX_SECTOR_SIZE_I8 }.rev() {
        let mut row_string = String::from("");
        row_string.push_str(format!("{} {bc}{BORDER_VERT}{COLOR_RESET}", yy).as_str());

        for xx in 0..MAX_SECTOR_SIZE_I8 {
            let cur_sector_info: Entity =
                crate::manifest::find_actual_sector_info(&qi_vec, (xx, yy));
            let cur_sector_type: EntityType = cur_sector_info.get_sector_type();
            match cur_sector_type {
                EntityType::Empty => row_string.push_str(format!(" {}", ".").as_str()),
                EntityType::PlayerShip => {
                    // print!("E");
                    row_string.push_str(format!(" {}", "E".bold().green()).as_str())
                }
                EntityType::Klingon => {
                    row_string.push_str(format!(" {}", "k".black().on_red()).as_str())
                }
                EntityType::Star => row_string.push_str(format!(" *").as_str()),
                EntityType::Romulan => {
                    row_string.push_str(format!(" {}", "r".black().on_yellow()).as_str())
                }
                EntityType::Planet => row_string.push_str(format!(" {}", "p".on_blue()).as_str()),
                EntityType::Starbase => row_string.push_str(format!(" {}", "B".cyan()).as_str()),
                EntityType::KilledKlingon => row_string.push_str(format!(" {}", "k").as_str()),
                EntityType::KilledRomulan => {
                    //print!("r");
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
                        " {:<20} {bc}{BORDER_VERT}{COLOR_RESET}",
                        crate::helpers::alert_status_of_quadrant(&qi_vec)
                    )
                    .as_str(),
                );
            }
            7 | 5 | 4 | 3 => {
                row_string.push_str(
                    format!("{}{bc}{BORDER_VERT}{COLOR_RESET}", human_out.remove(0)).as_str(),
                );
            }
            _ => {
                row_string.push_str(
                    format!("{:>12} ┃  {:>21}{bc}{BORDER_VERT}{COLOR_RESET}", " ", " ").as_str(),
                );
            }
        }
        println!("{}", row_string);
    }
    //
    println!("  {bc}{BORDER_LL}{BORDER_HORZ_60}{BORDER_LR}{COLOR_RESET}");
    //
    // put in axis points
    let mut row_string = String::from("");
    row_string.push_str(format!("    0 1 2 3 4 5 6 7 8 9  ").as_str());
    println!("{}", row_string);
}
