#![warn(missing_docs)]
//! # ui
//!
//! This is stuff related to the user interface (ui)

//use serde_json::to_string;
use colored::Colorize;
use std::io::{stdin, stdout, Write};

use crate::enums::{CmdType, SectorType};
use crate::gamedata::GameWideData;
//use crate::location::Location;
//use crate::sector::SectorInfo;
use crate::ship::ShipInfo;

// ==========================================================================
/// # command_processor
///
/// Process the user input command_processor
///
pub fn command_processor() {
    //
    let mut startup_cmd_vec: Vec<String> = Vec::new();
    startup_cmd_vec.push("stat".to_string());
    startup_cmd_vec.push("srs".to_string());

    let mut test_cmds_vec: Vec<String> = Vec::new();
    let mut cmd_part2_vec: Vec<String> = Vec::new();

    // let mut galaxy_vec: Vec<SectorInfo> = crate::galaxy::create_galaxy_vec();

    let mut g_info: GameWideData = GameWideData::new();
    g_info.end_star_date = g_info.cur_star_date + 131;
    g_info.gal_vec = crate::gamedata::construct_galaxy();

    let g_tmp = g_info.clone();
    let tmp_loc_list = crate::gamedata::create_vec_of_type(&g_tmp.gal_vec, SectorType::Starbase);
    let tmp_loc_quad = tmp_loc_list[0].get_location().gen_quad_tuple();
    g_info.charted[tmp_loc_quad.0 as usize][tmp_loc_quad.1 as usize] = true;

    // find the enterprise in the galaxy.  It should be only in one secor of one quadrant.
    //let g_tmp = g_info.clone();
    let tmp_loc_list = crate::gamedata::create_vec_of_type(&g_tmp.gal_vec, SectorType::Enterprise);
    g_info.enterprise = crate::ship::ShipInfo::new();
    g_info
        .enterprise
        .set_location(tmp_loc_list[0].get_location());

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
        //let cmd2 = ;

        let cmd_vector: Vec<String> = cmd_input
            .to_lowercase()
            .replace(",", " ")
            .as_str()
            .split_whitespace()
            .map(str::to_string)
            .collect();

        // find the enterprise in the galaxy.  It should be only in one secor of one quadrant.
        //let enterprise_sector_info_vec =
        //crate::gamedata::create_vec_of_type(g_info.clone().gal_vec, SectorType::Enterprise);
        // enterprise_sector_info = enterprise_sector_info_vec[0];

        //let qi_vec = crate::helpers::create_qi_vec(
        //    &galaxy_vec,
        //    enterprise_sector_info_vec[0].get_location(),
        //);
        // println!(" qi_vec containing enterprise= {:?}\n", qi_vec);

        let cmd_type: CmdType = determine_cmd_type(cmd_vector[0].to_string());
        match cmd_type {
            CmdType::Phaser => {
                // println!("{:?}", cmd_vector);
                let res = crate::weapon::fire_phaser(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        let mut iterator = g_info.gal_vec.iter_mut();
                        while let Some(si) = iterator.next() {
                            let n_info = *si;
                            if n_info.get_location().is_same(killed_si.get_location()) {
                                si.set_sector_type(killed_si.get_sector_type());
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
                let res = crate::weapon::fire_torpedoe(&g_info, &cmd_vector);
                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.as_ref().unwrap().0;
                        let killed_si = res.as_ref().unwrap().1;
                        let mut iterator = g_info.gal_vec.iter_mut();
                        while let Some(si) = iterator.next() {
                            let n_info = *si;
                            if n_info.get_location().is_same(killed_si.get_location()) {
                                si.set_sector_type(killed_si.get_sector_type());
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

            /*
                  } else if cmd_vector[0].starts_with("tor") {
                        let mut orig_enterprise_quad_info: QuadInfo = game_info.get_enterprise_quad_info();
                        let mut orig_enterprise_sect_info: SectorInfo =
                            orig_enterprise_quad_info.find_enterprise_sector_info();
                        let mut tgt_enemy_info: SectorInfo =
                            SectorInfo::create(SectCoord::create_dummy(), SectorType::Empty);

                        let res = determine_target_sector(game_info, &cmd_vector);
                        match res {
                            Ok(_) => {
                                tgt_enemy_info = res.unwrap();
                            }
                            Err(e) => {
                                println!("{}: {:?}", crate::ui::red_error(), e);
                            }
                        }
                        //println!("{}: {:?}",crate::ui::red_error(),tgt_enemy_info);

                        let mut can_target = tgt_enemy_info.obj_type != SectorType::Empty;

                        if can_target {
                            let path_res = SectorInfo::chk_straight_line_path(
                                orig_enterprise_quad_info,
                                orig_enterprise_sect_info.get_sector_loc(),
                                tgt_enemy_info.get_sector_loc(),
                            );
                            match path_res {
                                Ok(_) => {
                                    can_target = path_res.unwrap();
                                    if can_target {
                                    } else {
                                        println!("Error: No enemy target found at {:?}", tgt_enemy_info);
                                    }
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                            if can_target {
                                println!("ATTACK {:?}", tgt_enemy_info);
                                game_info.inc_enemy_killed(tgt_enemy_info.obj_type);
                                tgt_enemy_info.obj_type = SectorType::Empty;
                                //tgt_enemy_info.obj_energy = 0;

                                orig_enterprise_quad_info.update_sector_info(tgt_enemy_info);
                                game_info.update_enterprise_quad_info(orig_enterprise_quad_info, false);
                                //println!("After attack {:?}", tgt_enemy_info);
                            }

                            // short_range(*game_info);
                        } else {
                            println!("Error:  command format should be  tor x y");
                        }
            */
            CmdType::Jump => {
                let res = crate::movement::jump_enterprise(&g_info, &cmd_vector);

                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.unwrap();
                        //let mut g_tmp = g_info.clone();
                        let mut iterator = g_info.gal_vec.iter_mut();
                        while let Some(si) = iterator.next() {
                            if si.get_sector_type() == SectorType::Enterprise {
                                si.set_location(updated_enterprise.get_location());
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
                let res = crate::movement::move_enterprise(&g_info, &cmd_vector);

                match res {
                    Ok(_) => {
                        let updated_enterprise: ShipInfo = res.unwrap();
                        let mut iterator = g_info.gal_vec.iter_mut();
                        while let Some(si) = iterator.next() {
                            if si.get_sector_type() == SectorType::Enterprise {
                                si.set_location(updated_enterprise.get_location());
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
                crate::disp::long_range_sensor_disp(&g_info);
            }
            CmdType::SRS => {
                let cur_loc = g_info.enterprise.get_location().gen_quad_tuple();
                for x_delta in -1i8..=1i8 {
                    for y_delta in -1i8..=1i8 {
                        let mut x: i8 = cur_loc.0 + x_delta;
                        let mut y: i8 = cur_loc.1 + y_delta;

                        if x < 0 {
                            x = 0;
                        } else if x >= crate::constants::MAX_GALAXY_SIZE_I8 - 1i8 {
                            x = crate::constants::MAX_GALAXY_SIZE_I8 - 1i8;
                        }

                        if y < 0 {
                            y = 0;
                        } else if y >= crate::constants::MAX_GALAXY_SIZE_I8 - 1i8 {
                            y = crate::constants::MAX_GALAXY_SIZE_I8 - 1i8;
                        }
                        //println!("{} {}",x,y);
                        g_info.charted[x as usize][y as usize] = true;
                    }
                }
                crate::disp::short_range_sensor_disp(&g_info);
            }
            CmdType::Status => {
                crate::disp::game_stat_disp(&g_info);
            }
            CmdType::Test => {
                test_cmds_vec.push("stat".to_string());
                test_cmds_vec.push("srs".to_string());
                test_cmds_vec.push("lrs".to_string());
                test_cmds_vec.push("quit".to_string());
            }
            CmdType::Help => {
                crate::disp::help_screen();
            }
            CmdType::Quit => {
                println!("Thanks for playing");
                break 'proccess_loop;
            }
            _ => {
                println!(
                    "{}  command \"{}\" -- not understood",
                    red_syntax_error(),
                    cmd_input.trim()
                );
            }
        }
        // println!("at bottom {:?}",enterprise);
    }
}

pub fn determine_cmd_type(cmd_string: String) -> CmdType {
    if cmd_string.starts_with("qui") || cmd_string.starts_with("exit") {
        return CmdType::Quit;
    } else if cmd_string.starts_with("tor") {
        return CmdType::Torpedoe;
    } else if cmd_string.starts_with("pha") {
        return CmdType::Phaser;
    } else if cmd_string.starts_with("jum") {
        return CmdType::Jump;
    } else if cmd_string.starts_with("mov") {
        return CmdType::Move;
    } else if cmd_string.starts_with("lrs") {
        return CmdType::LRS;
    } else if cmd_string.starts_with("srs") {
        return CmdType::SRS;
    } else if cmd_string.starts_with("stat") {
        return CmdType::Status;
    } else if cmd_string.starts_with("test") {
        return CmdType::Test;
    } else if cmd_string.starts_with("?") || cmd_string.starts_with("hel") {
        return CmdType::Help;
    } else {
        return CmdType::Empty;
    }
}

// ==========================================================================
/// # disp_title
///
///
pub fn disp_title(title: &str) {
    let tmp_title = format!("{}", title.underline());
    println!("\n{: ^45}", tmp_title);
}

// ==========================================================================
/// # red_error
///
/// put the
///
pub fn red_error() -> String {
    "Error:".underline().bright_red().to_string()
}

// ==========================================================================
/// # red_syntax_error
///
pub fn red_syntax_error() -> String {
    "Syntax Error:".underline().bright_red().to_string()
}
