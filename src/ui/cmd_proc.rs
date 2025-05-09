#![warn(missing_docs)]
//! # cmd_proc.rs
//!
//! User Command processing functions

use crate::astro::AstroType;
use crate::constants::MAX_GALAXY_SIZE_I8; //, MAX_SECTOR_SIZE_I8};

use colored::Colorize;

use crate::ui::{BORDER_COLOR_GREEN, COLOR_RESET};

use std::io::{stdin, stdout, Write};

// ==========================================================================
/// # determine_cmd_type
///
pub fn determine_cmd_type(cmd_string: String) -> CmdType {
    if cmd_string.starts_with("tor") {
        CmdType::Torpedoe
    } else if cmd_string.starts_with("pha") {
        CmdType::Phaser
    } else if abbrev(&cmd_string, "jum", "jump") {
        CmdType::Jump
    } else if abbrev(&cmd_string, "mov", "move") {
        CmdType::Move
    } else if abbrev(&cmd_string, "lrs", "lrs") {
        CmdType::Lrs
    } else if abbrev(&cmd_string, "srs", "srs") {
        CmdType::Srs
    } else if abbrev(&cmd_string, "stat", "status") {
        CmdType::Status
    } else if abbrev(&cmd_string, "save", "save") {
        CmdType::Save
    } else if abbrev(&cmd_string, "rest", "restore") {
        CmdType::Restore
    } else if abbrev(&cmd_string, "test", "test") {
        CmdType::Test
    } else if cmd_string.starts_with("qui") || cmd_string.starts_with("exit") {
        CmdType::Quit
    } else if cmd_string.starts_with("?") || cmd_string.starts_with("hel") {
        CmdType::Help
    } else if abbrev(&cmd_string, "comment", "comment") {
        CmdType::Comment
    //} else if abbrev(&cmd_string, "recordoff", "recordoff") {
    //    CmdType::RecordOff
    } else {
        CmdType::Empty
    }
}

#[inline]
pub fn abbrev(what: &String, least: &str, full: &str) -> bool {
    //! Check if `what` is an abbreviation of `full` and starts with `least`.
    what.starts_with(least) && full.contains(what)
}
// =========================================================
/// # command_processor
///
/// Process the user input command_processor
///
pub fn command_processor(startup_commands: &[String]) {
    //let mut recording_commands: bool = false;
    //
    let mut startup_cmd_vec: Vec<String> = startup_commands.to_vec(); //Vec::new();
                                                                     //println!("{:?}",startup_cmd_vec);
                                                                     //let mut test_cmds_vec: Vec<String> = Vec::new();
    let mut cmd_part2_vec: Vec<String> = Vec::new();

    //startup_cmd_vec.push("stat".to_string());
    //startup_cmd_vec.push("srs".to_string());

    //test_cmds_vec.push("save".to_string());
    //test_cmds_vec.push("restore".to_string());
    //test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 1 1".to_string());

    let mut g_info: crate::manifest::Manifest = crate::manifest::Manifest::new();
    g_info.end_star_date = g_info.cur_star_date + 131;

    // Actually construct the galaxy
    g_info.uni_map = crate::manifest::Manifest::construct_galaxy((0, 0));

    let g_tmp = g_info.clone();
    //
    let tmp_loc_list = crate::manifest::isolate_type(&g_tmp, AstroType::Starbase);
    let tmp_loc_quad = tmp_loc_list[0].ret_quad_tuple();
    g_info.charted[tmp_loc_quad.0 as usize][tmp_loc_quad.1 as usize] = true;

    // find the enterprise in the galaxy.
    let tmp_loc_list = crate::manifest::isolate_type(&g_tmp, AstroType::PlayerShip);
    g_info.player_ship = crate::ship_info::PlayerShip::new();
    g_info.player_ship.set_entity(tmp_loc_list[0]);

    'process_loop: loop {
        let mut cmd_input: String = String::new();

        if startup_cmd_vec.is_empty() && cmd_part2_vec.is_empty() {
            print!(" Command:");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut cmd_input)
                .expect("Failed to read line");
        } else if !startup_cmd_vec.is_empty() {
            cmd_input = startup_cmd_vec.remove(0);
        //        } else if !test_cmds_vec.is_empty() {
        //            cmd_input = test_cmds_vec.remove(0);
        //           println!(" === auto test cmd ===  {}", cmd_input);
        } else {
            cmd_input = cmd_part2_vec.remove(0);
        }

        let cmd_vector: Vec<String> = cmd_input
            .to_lowercase()
            .replace(",", " ")
            .as_str()
            .split_whitespace()
            .map(str::to_string)
            .collect();

        //println!("{:?}",cmd_vector);    

        let cmd_type: CmdType = determine_cmd_type(cmd_vector[0].to_string());

        // make sure proper sectors are explored
        let cur_loc = g_info.player_ship().get_entity().ret_quad_tuple();
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

        // the actual command executor
        match cmd_type {
            // Phaser
            CmdType::Phaser => {
                // println!("{:?}", cmd_vector);
                let res = crate::ship_info::weapon::fire_phaser(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: crate::ship_info::PlayerShip =
                            res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        println!("killed {:?} with phaser UI", killed_si);

                        //killed_si.kill_enemy();
                        //let g_tmp = g_info.galaxy_vec.clone();
                        g_info.uni_map.remove(&killed_si.to_key_string());
                        g_info.uni_map.insert(killed_si.to_key_string(), killed_si);
                        g_info.player_ship = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.is_empty() {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                }
            }

            CmdType::Torpedoe => {
                let res = crate::ship_info::weapon::fire_torpedoe(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: crate::ship_info::PlayerShip =
                            res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        println!("killed {:?} with torpedoe UI", killed_si);
                        g_info.uni_map.remove(&killed_si.to_key_string());
                        g_info.uni_map.insert(killed_si.to_key_string(), killed_si);
                        g_info.player_ship = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.is_empty() {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                }
            }

            CmdType::Jump => {
                let res = crate::ship_info::movement::jump_enterprise(&g_info, &cmd_vector);

                match res {
                    Ok(_) => {
                        let updated_enterprise: crate::ship_info::PlayerShip = res.unwrap();
                        g_info
                            .uni_map
                            .remove(&g_info.player_ship.get_entity().to_key_string());
                        g_info.uni_map.insert(
                            updated_enterprise.get_entity().to_key_string(),
                            updated_enterprise.get_entity(),
                        );

                        g_info.player_ship = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.is_empty() {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                };
            }

            CmdType::Move => {
                let res = crate::ship_info::movement::move_enterprise(&g_info, &cmd_vector);

                match res {
                    Ok(_) => {
                        let updated_enterprise: crate::ship_info::PlayerShip = res.unwrap();
                        g_info
                            .uni_map
                            .remove(&g_info.player_ship.get_entity().to_key_string());
                        g_info.uni_map.insert(
                            updated_enterprise.get_entity().to_key_string(),
                            updated_enterprise.get_entity(),
                        );

                        g_info.player_ship = updated_enterprise;
                        // auto perform a short range scan
                        if cmd_part2_vec.is_empty() {
                            cmd_part2_vec.push("srs".to_string());
                        }
                    }
                    Err(e) => {
                        println!("{} {:?}", red_error(), e);
                    }
                };
            }
            CmdType::Lrs => {
                crate::ui::lrs::long_range_sensor_disp(&g_info);
            }
            CmdType::Srs => {
                crate::ui::srs::short_range_sensor_disp(&g_info);
            }
            CmdType::Status => {
                crate::ui::stat_screen::game_stat_disp(&g_info);
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
                if cmd_part2_vec.is_empty() {
                    cmd_part2_vec.push("stat".to_string());
                }
            }
            CmdType::Save => {
                crate::manifest::freeze(&g_info, &cmd_vector);
            }
            CmdType::Help => {
                crate::ui::help_screen::help_screen(&g_info);
            }
            CmdType::Quit => {
                break 'process_loop;
            }
            CmdType::Comment => {
                println!("==> {BORDER_COLOR_GREEN}{}{COLOR_RESET} <==", cmd_vector.join(" "));
                //recording_commands = true;
            }
            //CmdType::RecordOff => {
            //    println!("Command recording OFF");
            //    recording_commands = false;
            //}
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

// =====================================================================
/// #CmdType
///  The type of commands
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CmdType {
    Help,
    Quit,
    Srs,
    Lrs,
    Phaser,
    Torpedoe,
    Move,
    Jump,
    Test,
    Status,
    Restore,
    Save,
    Comment,
//    RecordOff,
    Empty,
}
