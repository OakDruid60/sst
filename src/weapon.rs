#![warn(missing_docs)]
//! # weapon
//!
//! This is stuff related to weapons

use crate::constants::MAX_SECTOR_SIZE_I8;
use crate::gamedata::GameWideData;
use crate::sector::SectorInfo;
use crate::ship::ShipInfo;
/*

/// # lrs_vicinity_helper
///
/// A helper function for the [short_range] function.  Formats only one linr of a
/// long randge scan using only the sectors in the vicinity.   Also handles marking the location
/// of the ENTERPRISE.
///
pub fn lrs_vicinity_helper(dy: isize, game_info: &Details) -> String {
    //
    //let gm = game_info;
    let enterprise = game_info.get_enterprise();
    //let row_string: &str = "hh";
    let mut row_string: String = String::from("hh");

    for dx in -1..=1 {
        println!("{} {}", dx, dy);
        if crate::galaxy::validate_nearby_quadrant(enterprise, dx, dy) {
            /*   let test_x: usize =
                 (enterprise.get_quad_coord().get_x() as isize + dx) as usize;
             let test_y: usize =
                 (enterprise.get_quad_coord().get_y() as isize + dy) as usize;
             //    return self.quad_info_array[test_x][test_y];

             let mut tmp_quad_info = gm.get_quad_info_at_xy(test_x, test_y);
             let tmp_summary = tmp_quad_info.summarize();
             let tmp_visited: bool = tmp_quad_info.visited_flag();

            if dx == 0 && dy == 0 {
                 // i.e. where the ENTERPRISE is.
                 let tmp = format!(">{}<", tmp_summary.long_range_quad_encoded(tmp_visited));
                 row_string.push_str(tmp.as_str());
             } else {
                 let tmp = format!(" {} ", tmp_summary.long_range_quad_encoded(tmp_visited));
                 row_string.push_str(tmp.as_str());
             }
             */
        } else {
            row_string.push_str(format!("{:7}", "xx").as_str());
        }
    }

    let bb = row_string.to_string();
    return bb; //row_string.to_string();
}

// ==========================================================================
/// # determine_target_sector
///
///
pub fn determine_target_sector(
    game_info: &mut Details,
    cmd_vector: &Vec<String>,
) -> Result<SectorInfo, String> {
    if cmd_vector.len() == 1 {
        let res = SectorInfo::locate_nearest_possible_enemy(*game_info);
        let mut tgt_enemy_info = SectorInfo::create(SectCoord::create_dummy(), SectorType::Empty);

        match res {
            Ok(_) => tgt_enemy_info = res.unwrap(),
            Err(e) => return Err(e),
        }
        if tgt_enemy_info.obj_type != SectorType::Empty {
            return Ok(tgt_enemy_info);
        } else {
            return Err("no found nearest".to_string());
        }
    } else if cmd_vector.len() == 3 {
        let res = SectorInfo::validate_sector(cmd_vector);
        match res {
            Ok(_) => {
                let tgt_enemy_info = SectorInfo::create(res.unwrap(), SectorType::Empty);

                let bad_guys: Vec<SectorInfo> =
                    SectorInfo::create_vector_of_possible_enemy_sector_info(*game_info);

                for potential_enemy_info in bad_guys.iter() {
                    if tgt_enemy_info
                        .get_sector_loc()
                        .is_same_place(potential_enemy_info.get_sector_loc())
                    {
                        return Ok(*potential_enemy_info);
                    }
                }
                return Err("no enemy target found ".to_string());
            }
            Err(e) => return Err(e),
        }
    } else {
        return Err("no no no no".to_string());
    }
}

// ==========================================================================
/// # command_processor
///
/// Process the user input command_processor
///
pub fn command_processor(galaxy_vec: &mut Vec<SectorInfo>) {
    //
    loop {
        let mut cmd_input: String = String::new();
        println!(" Command:");
        std::io::stdin()
            .read_line(&mut cmd_input)
            .expect("Failed to read line");
        let cmd2 = cmd_input.to_lowercase().replace(",", " ");

        let cmd_vector: Vec<String> = cmd2
            .as_str()
            .split_whitespace()
            .map(str::to_string)
            .collect();

        // find the enterprise in the galaxy.  It should be only in one secor of one quadrant.
        let enterprise_sector_info_vec =
            crate::galaxy::create_vec_of_type(&galaxy_vec, SectorType::Enterprise);
        // println!("\n\n enterprise_sector_info_vec= {:?}\n\n",enterprise_sector_info_vec);

        let mut enterprise_sector_info = enterprise_sector_info_vec[0];
        // println!(" enterprise_sector_info= {:?}\n", enterprise_sector_info);

        let mut enterprise = crate::ship::create_enterprise(enterprise_sector_info);
        // println!(" enterprise= {:?}\n\n", enterprise);

        let mut qi_vec = crate::helpers::create_qi_vec(
            &galaxy_vec,
            enterprise_sector_info_vec[0].get_uni_coord(),
        );
        // println!(" qi_vec containing enterprise= {:?}\n", qi_vec);
        if cmd_vector[0].starts_with("tor") {
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
        } else if cmd_vector[0].starts_with("mov") {
            if cmd_vector.len() == 3 {
                let res = SectorInfo::validate_sector(&cmd_vector);
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                //   let direction: f64 = cmd_vector[1]
                //    .trim()
                //    .parse()
                //    .expect("Please type a number for the direction (1.0-9.0)!");
                //let distance: f64 = cmd_vector[2]
                //    .trim()
                //    .parse()
                //    .expect("Please type a number for the distance!");

                //let mut enterprise_quad_info: QuadInfo = game_info.get_enterprise_quad_info();
                //    crate::quadrant::QuadInfo::move_enterprise_to_new_location_in_sector(
                //        &mut enterprise_quad_info,
                //        direction,
                //        distance,
                //    );
                //   game_info.update_enterprise_quad_info(enterprise_quad_info);
                //short_range(*game_info);
            } else {
                println!("Error:  command format should be  mov x y");
            }
        } else {
            println!("Error:  command not understood");
        }
    }
}
*/

// =================================================================
/// # fire_torpedoe
///
///
pub fn fire_torpedoe(
    g_info: &GameWideData,
    cmd_vector: &Vec<String>,
) -> Result<(ShipInfo, SectorInfo), String> {
    if cmd_vector.len() == 3 {
        if g_info.enterprise.get_torpedoes() <= 0 {
            return Err(
                "There are no torpedoes available to be used.  You need to dock at a Starbase."
                    .to_string(),
            );
        }
        let res = crate::helpers::validate_x_y_input(&cmd_vector, MAX_SECTOR_SIZE_I8);
        let mut n_info: SectorInfo;
        match res {
            Ok(_) => {
                let tgt_sector = res.unwrap();
                let qi_vec = g_info.create_quadrant_vec(g_info.enterprise.get_location());
                //
                let mut potential_bad_guys = crate::gamedata::create_bad_guy_qi_vec(
                    &qi_vec,
                    g_info.enterprise.get_location(),
                    false,
                );
                if potential_bad_guys.len() == 0 {
                    return Err(format!("No enemy targets found in current quadrant").to_string());
                }
                let mut found_it = false;
                for si in potential_bad_guys.iter_mut() {
                    n_info = *si;
                    if n_info.get_location().is_same_sect_tuple(tgt_sector) {
                        found_it = true;
                        break;
                    }
                }
                if !found_it {
                    return Err(format!("No enemy target found at {:?}", tgt_sector).to_string());
                }
                potential_bad_guys = crate::gamedata::create_bad_guy_qi_vec(
                    &qi_vec,
                    g_info.enterprise.get_location(),
                    true,
                );
                if potential_bad_guys.len() == 0 {
                    return Err(format!("No enemy target found along a clear straight line path from the Enterprise to {:?}",tgt_sector).to_string());
                } else {
                    //let mut found_it: bool = false;
                    for si in potential_bad_guys.iter_mut() {
                        n_info = *si;
                        if n_info.get_location().is_same_sect_tuple(tgt_sector) {
                            println!(
                                "Killed the {:?} at {:?} with torpedoe",
                                n_info.get_sector_type(),
                                tgt_sector
                            );
                            n_info.kill_enemy(); //potential_bad_guys[0].kill_enemy();
                            let mut updated_enterprise = g_info.enterprise;
                            updated_enterprise.use_torpedoe();
                            return Ok((updated_enterprise, n_info));
                        }
                    }
                }
                return Err(format!(
                    "A clear, straight line path to {:?} does not exist",
                    tgt_sector
                )
                .to_string());
            }
            //
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        Err("Command format should be  tor x y".to_string())
    }
}

// =================================================================
/// # fire_phaser
///
///
pub fn fire_phaser(
    g_info: &GameWideData,
    cmd_vector: &Vec<String>,
) -> Result<(ShipInfo, SectorInfo), String> {
    if cmd_vector.len() == 3 {
        if g_info.enterprise.get_energy() <= 0 {
            return Err(
                "No energy is available to be used.  You need to dock at a Starbase.".to_string(),
            );
        }
        let res = crate::helpers::validate_x_y_input(&cmd_vector, MAX_SECTOR_SIZE_I8);
        let mut n_info: SectorInfo;
        match res {
            Ok(_) => {
                let tgt_sector = res.unwrap();
                let qi_vec = g_info.create_quadrant_vec(g_info.enterprise.get_location());
                //println!("{:?}", tgt_sector_tuple);
                let mut potential_bad_guys = crate::gamedata::create_bad_guy_qi_vec(
                    &qi_vec,
                    g_info.enterprise.get_location(),
                    false,
                );
                if potential_bad_guys.len() == 0 {
                    return Err(format!("No enemy targets found in current quadrant").to_string());
                }
                let mut found_it = false;
                for si in potential_bad_guys.iter_mut() {
                    n_info = *si;
                    if n_info.get_location().is_same_sect_tuple(tgt_sector) {
                        found_it = true;
                        break;
                    }
                }
                if !found_it {
                    return Err(format!("No enemy target found at {:?}", tgt_sector).to_string());
                }
                potential_bad_guys = crate::gamedata::create_bad_guy_qi_vec(
                    &qi_vec,
                    g_info.enterprise.get_location(),
                    true,
                );
                if potential_bad_guys.len() == 0 {
                    return Err(format!("No enemy target found along a clear straight line path from the Enterprise to {:?}",tgt_sector).to_string());
                } else {
                    //let mut found_it: bool = false;
                    for si in potential_bad_guys.iter_mut() {
                        n_info = *si;
                        if n_info.get_location().is_same_sect_tuple(tgt_sector) {
                            println!(
                                "Killed the {:?} at {:?} with phaser",
                                n_info.get_sector_type(),
                                tgt_sector
                            );
                            n_info.kill_enemy(); //potential_bad_guys[0].kill_enemy();
                            let mut updated_enterprise = g_info.enterprise;
                            updated_enterprise.use_energy(1000);
                            return Ok((updated_enterprise, n_info));
                        }
                    }
                }
                return Err(format!(
                    "A clear, straight line path to {:?} does not exist",
                    tgt_sector
                )
                .to_string());
            }
            //
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        Err("Command format should be  pha x y".to_string())
    }
}
