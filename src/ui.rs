#![warn(missing_docs)]
//! # ui.rs
//!
//! This is stuff related to the user interface (ui)

//use serde_json::to_string;
use colored::Colorize;
use std::io::{stdin, stdout, Write};

pub mod help;
pub mod logo;
pub mod lrs;
pub mod misc;
pub mod srs;

use crate::enterprise::ShipInfo;
use crate::manifest::constants::MAX_GALAXY_SIZE_I8;
use crate::manifest::enums::{CmdType, SectorType};
use crate::manifest::Manifest;

pub const BORDER_COLOR_RED: &str = "\x1b[91m";
pub const BORDER_COLOR_YELLOW: &str = "\x1b[93m";
pub const BORDER_COLOR_GREEN: &str = "\x1b[92m";
pub const BORDER_COLOR_CYAN: &str = "\x1b[96m";
//pub const BORDER_COLOR: &str = "\x1b[91m";
pub const BORDER_HORZ_60: &str = "════════════════════════════════════════════════════════════";
pub const BORDER_VERT: &str = "║";
pub const BORDER_UR: &str = "╗";
pub const BORDER_UL: &str = "╔";
pub const BORDER_LR: &str = "╝";
pub const BORDER_LL: &str = "╚";
pub const BORDER_MR: &str = "╣";
pub const BORDER_ML: &str = "╠";

pub const COLOR_RESET: &str = "\x1b[0m";

// =========================================================
/// # command_processor
///
/// Process the user input command_processor
///
pub fn command_processor() {
    let mut recording_commands: bool = false;
    //
    let mut startup_cmd_vec: Vec<String> = Vec::new();
    //startup_cmd_vec.push("restore".to_string());
    startup_cmd_vec.push("stat".to_string());
    startup_cmd_vec.push("srs".to_string());

    let mut test_cmds_vec: Vec<String> = Vec::new();
    let mut cmd_part2_vec: Vec<String> = Vec::new();

    let mut g_info: Manifest = Manifest::new();
    g_info.end_star_date = g_info.cur_star_date + 131;
    g_info.gal_vec = crate::manifest::construct_galaxy();

    let g_tmp = g_info.clone();
    let tmp_loc_list = crate::manifest::create_vec_of_type(&g_tmp.gal_vec, SectorType::Starbase);
    let tmp_loc_quad = tmp_loc_list[0].create_quad_tuple();
    g_info.charted[tmp_loc_quad.0 as usize][tmp_loc_quad.1 as usize] = true;

    // find the enterprise in the galaxy.
    let tmp_loc_list = crate::manifest::create_vec_of_type(&g_tmp.gal_vec, SectorType::Enterprise);
    g_info.enterprise = crate::enterprise::ShipInfo::new();
    g_info.enterprise.set_entity(tmp_loc_list[0].clone());

    'proccess_loop: loop {
        let mut cmd_input: String = String::new();

        if startup_cmd_vec.len() == 0 && test_cmds_vec.len() == 0 && cmd_part2_vec.len() == 0 {
            print!(" Command:");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut cmd_input)
                .expect("Failed to read line");
        } else {
            if startup_cmd_vec.len() > 0 {
                cmd_input = startup_cmd_vec.remove(0);
            } else {
                if test_cmds_vec.len() > 0 {
                    cmd_input = test_cmds_vec.remove(0);
                    println!(" === auto test cmd ===  {}", cmd_input);
                } else {
                    cmd_input = cmd_part2_vec.remove(0);
                }
            }
        }

        let cmd_vector: Vec<String> = cmd_input
            .to_lowercase()
            .replace(",", " ")
            .as_str()
            .split_whitespace()
            .map(str::to_string)
            .collect();

        if recording_commands {
            if cmd_input.ends_with("\n") {
                g_info.test_cmds_vec.push(cmd_input.clone());
            }
        }

        let cmd_type: CmdType = determine_cmd_type(cmd_vector[0].to_string());
        match cmd_type {
            // Phaser
            CmdType::Phaser => {
                // println!("{:?}", cmd_vector);
                let res = crate::enterprise::weapon::fire_phaser(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        println!("killed {:?} with phaser UI", killed_si);

                        //killed_si.kill_enemy();
                        let g_tmp = g_info.gal_vec.clone();
                        for (pos, e) in g_tmp.iter().enumerate() {
                            if e.is_same(&killed_si) {
                                g_info.gal_vec.remove(pos);
                                g_info.gal_vec.push(killed_si);
                                break;
                            }
                        }
                        g_info.enterprise = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.len() == 0 {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                }
            }

            CmdType::Torpedoe => {
                let res = crate::enterprise::weapon::fire_torpedoe(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        println!("killed {:?} with torpedoe UI", killed_si);
                        let g_tmp = g_info.gal_vec.clone();
                        for (pos, e) in g_tmp.iter().enumerate() {
                            if e.is_same(&killed_si) {
                                g_info.gal_vec.remove(pos);
                                g_info.gal_vec.push(killed_si);
                                break;
                            }
                        }
                        g_info.enterprise = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.len() == 0 {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                }
            }

            CmdType::Jump => {
                let res = crate::enterprise::movement::jump_enterprise(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.unwrap();
                        let g_tmp = g_info.gal_vec.clone();
                        for (pos, e) in g_tmp.iter().enumerate() {
                            if e.get_sector_type() == SectorType::Enterprise {
                                g_info.gal_vec.remove(pos);
                                g_info.gal_vec.push(updated_enterprise.get_entity());
                                break;
                            }
                        }
                        g_info.enterprise = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.len() == 0 {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                };
            }

            CmdType::Move => {
                let res = crate::enterprise::movement::move_enterprise(&g_info, &cmd_vector);

                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.unwrap();
                        let g_tmp = g_info.gal_vec.clone();
                        for (pos, e) in g_tmp.iter().enumerate() {
                            if e.get_sector_type() == SectorType::Enterprise {
                                g_info.gal_vec.remove(pos);
                                g_info.gal_vec.push(updated_enterprise.get_entity());
                                break;
                            }
                        }
                        g_info.enterprise = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.len() == 0 {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                };
            }
            CmdType::LRS => {
                let cur_loc = g_info.enterprise.get_entity().create_quad_tuple();
                for x_delta in -1i8..=1i8 {
                    for y_delta in -1i8..=1i8 {
                        let mut x: i8 = cur_loc.0 + x_delta;
                        let mut y: i8 = cur_loc.1 + y_delta;

                        if x < 0 {
                            x = 0;
                        } else if x >= MAX_GALAXY_SIZE_I8 - 1i8 {
                            x = MAX_GALAXY_SIZE_I8 - 1i8;
                        }

                        if y < 0 {
                            y = 0;
                        } else if y >= MAX_GALAXY_SIZE_I8 - 1i8 {
                            y = MAX_GALAXY_SIZE_I8 - 1i8;
                        }
                        //println!("{} {}",x,y);
                        g_info.charted[x as usize][y as usize] = true;
                    }
                }
                crate::ui::lrs::long_range_sensor_disp(&g_info);
            }
            CmdType::SRS => {
                let cur_loc = g_info.enterprise.get_entity().create_quad_tuple();
                for x_delta in -1i8..=1i8 {
                    for y_delta in -1i8..=1i8 {
                        let mut x: i8 = cur_loc.0 + x_delta;
                        let mut y: i8 = cur_loc.1 + y_delta;

                        if x < 0 {
                            x = 0;
                        } else if x >= MAX_GALAXY_SIZE_I8 - 1i8 {
                            x = MAX_GALAXY_SIZE_I8 - 1i8;
                        }

                        if y < 0 {
                            y = 0;
                        } else if y >= MAX_GALAXY_SIZE_I8 - 1i8 {
                            y = MAX_GALAXY_SIZE_I8 - 1i8;
                        }
                        //println!("{} {}",x,y);
                        g_info.charted[x as usize][y as usize] = true;
                    }
                }
                crate::ui::srs::short_range_sensor_disp(&g_info);
            }
            CmdType::Status => {
                crate::ui::misc::game_stat_disp(&g_info);
            }
            CmdType::Test => {
                //test_cmds_vec.push("stat".to_string());
                //test_cmds_vec.push("srs".to_string());
                //test_cmds_vec.push("lrs".to_string());
                //test_cmds_vec.push("restore".to_string());
                //test_cmds_vec.push("quit".to_string());
            }
            CmdType::Restore => {
                g_info = crate::manifest::thaw(&cmd_vector).expect("REASON");
                if g_info.test_cmds_vec.len() > 0 {
                    test_cmds_vec = g_info.test_cmds_vec.clone();
                    //println!("{:?}", test_cmds_vec);
                    g_info.test_cmds_vec = Vec::new();
                }
                if cmd_part2_vec.len() == 0 {
                    cmd_part2_vec.push("stat".to_string());
                }
            }
            CmdType::Save => {
                crate::manifest::freeze(&g_info, &cmd_vector);
            }
            CmdType::Help => {
                crate::ui::help::help_screen(&g_info);
            }
            CmdType::Quit => {
                break 'proccess_loop;
            }
            CmdType::RecordOn => {
                println!("Command recording ON");
                recording_commands = true;
            }
            CmdType::RecordOff => {
                println!("Command recording OFF");
                recording_commands = false;
            }
            _ => {
                println!(
                    "{}  command \"{}\" -- not understood",
                    red_syntax_error(),
                    cmd_input.trim()
                );
            }
        }
    }
}

// ==========================================================
/// # disp_title
///
/// New attempt at a command title bar
pub fn disp_title(title: &str, g_info: &Manifest, bc: &str) {
    //let tmp_title = format!("{}", title.underline())
    println!("");
    //let BORDER_COL: &str  = BORDER_COLOR_GREEN;
    println!("  {bc}{BORDER_UL}{BORDER_HORZ_60}{BORDER_UR}{COLOR_RESET}");
    println!(
        "  {bc}{BORDER_VERT}{COLOR_RESET} {: <30}{: >28} {bc}{BORDER_VERT}{COLOR_RESET}",
        title,
        g_info.enterprise.get_entity().to_compact_string()
    );
    println!("  {bc}{BORDER_ML}{BORDER_HORZ_60}{BORDER_MR}{COLOR_RESET}");
}

// ==========================================================
/// # red_error
/// put the

pub fn red_error() -> String {
    "Error:".underline().bright_red().to_string()
}

// ==========================================================
/// # red_syntax_error
///
pub fn red_syntax_error() -> String {
    "Syntax Error:".underline().bright_red().to_string()
}

// ==========================================================
/// # determine_cmd_type
///
pub fn determine_cmd_type(cmd_string: String) -> CmdType {
    if cmd_string.starts_with("tor") {
        return CmdType::Torpedoe;
    } else if cmd_string.starts_with("pha") {
        return CmdType::Phaser;
    } else if abbrev(&cmd_string, "jum", "jump") {
        return CmdType::Jump;
    } else if abbrev(&cmd_string, "mov", "move") {
        return CmdType::Move;
    } else if abbrev(&cmd_string, "lrs", "lrs") {
        return CmdType::LRS;
    } else if abbrev(&cmd_string, "srs", "srs") {
        return CmdType::SRS;
    } else if abbrev(&cmd_string, "stat", "status") {
        return CmdType::Status;
    } else if abbrev(&cmd_string, "save", "save") {
        return CmdType::Save;
    } else if abbrev(&cmd_string, "rest", "restore") {
        return CmdType::Restore;
    } else if abbrev(&cmd_string, "test", "test") {
        return CmdType::Test;
    } else if cmd_string.starts_with("q") || cmd_string.starts_with("exit") {
        return CmdType::Quit;
    } else if cmd_string.starts_with("?") || cmd_string.starts_with("hel") {
        return CmdType::Help;
    } else if abbrev(&cmd_string, "recordon", "recordon") {
        return CmdType::RecordOn;
    } else if abbrev(&cmd_string, "recordoff", "recordoff") {
        return CmdType::RecordOff;
    } else {
        return CmdType::Empty;
    }
}

#[inline]
pub fn abbrev(what: &String, least: &str, full: &str) -> bool {
    //! Check if `what` is an abbreviation of `full` and starts with `least`.
    return what.starts_with(&least) && full.contains(what);
}
