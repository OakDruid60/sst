#![warn(missing_docs)]
//! # cmd_proc.rs
//!
//! User Command processing functions

use crate::astro::AstroType;
use crate::manifest::constants::MAX_GALAXY_SIZE_I8; //, MAX_SECTOR_SIZE_I8};

use colored::Colorize;

use std::io::{stdin, stdout, Write};
// ==========================================================================
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
    } else if cmd_string.starts_with("qui") || cmd_string.starts_with("exit") {
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

    let mut g_info: crate::manifest::Manifest = crate::manifest::Manifest::new();
    g_info.end_star_date = g_info.cur_star_date + 131;

    // Actually construct the galaxy
    g_info.uni_map = crate::manifest::Manifest::construct_galaxy((0, 0));
    //println!("len = {:?}", g_info.uni_map().len());
    //println!("capacity = {:?}", g_info.uni_map().capacity());

    let g_tmp = g_info.clone();

    // fixme let tmp_loc_list = crate::manifest::create_vec_of_type(&g_tmp.galaxy_vec, AstroType::Starbase);
    //let tmp_loc_quad = tmp_loc_list[0].ret_quad_tuple();
    //g_info.charted[tmp_loc_quad.0 as usize][tmp_loc_quad.1 as usize] = true;

    // find the enterprise in the galaxy.
    let tmp_loc_list = crate::manifest::isolate_type(&g_tmp, AstroType::PlayerShip);
    g_info.player_ship = crate::ship_info::PlayerShip::new();
    g_info.player_ship.set_entity(tmp_loc_list[0].clone());

    //let japt1 = crate::manifest::isolate_quadrant(&g_tmp, &g_info.player_ship.get_entity());
    //for ao in japt1.iter() {
    //    println!("{}", ao);
    //}

    'proccess_loop: loop {
        let mut cmd_input: String = String::new();

        if startup_cmd_vec.len() == 0 && test_cmds_vec.len() == 0 && cmd_part2_vec.len() == 0 {
            print!(" Command:");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut cmd_input)
                .expect("Failed to read line");
        } else if startup_cmd_vec.len() > 0 {
            cmd_input = startup_cmd_vec.remove(0);
        } else {
            if test_cmds_vec.len() > 0 {
                cmd_input = test_cmds_vec.remove(0);
                println!(" === auto test cmd ===  {}", cmd_input);
            } else {
                cmd_input = cmd_part2_vec.remove(0);
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
                let res = crate::ship_info::weapon::fire_torpedoe(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: crate::ship_info::PlayerShip =
                            res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        println!("killed {:?} with torpedoe UI", killed_si);
                        g_info.uni_map.remove(&killed_si.to_key_string());
                        g_info.uni_map.insert(killed_si.to_key_string(), killed_si);
                        //let g_tmp = g_info.galaxy_vec.clone();
                        //for (pos, e) in g_tmp.iter().enumerate() {
                        //    if e.is_same(&killed_si) {
                        //        g_info.galaxy_vec.remove(pos);
                        //        g_info.galaxy_vec.push(killed_si);
                        //        break;
                        //    }
                        //}
                        g_info.player_ship = updated_enterprise;
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
                let res = crate::ship_info::movement::move_enterprise(&g_info, &cmd_vector);
                //println!("need to fix {:?}",res.unwrap());

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
                crate::ui::lrs::long_range_sensor_disp(&g_info);
            }
            CmdType::SRS => {
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
                crate::ui::help_screen::help_screen(&g_info);
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
