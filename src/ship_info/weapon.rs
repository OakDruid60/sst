#![warn(missing_docs)]
//! # weapon.rs
//!
//! This is stuff related to weapons of the Enterprise.

use crate::astro::{AstroObject, AstroType};
use crate::constants::MAX_SECTOR_SIZE_I8;
use crate::manifest::Manifest;
use crate::ship_info::PlayerShip;

// =================================================================
/// # fire_torpedoe
///
///
pub fn fire_torpedoe(
    g_info: &Manifest,
    cmd_vector: &[String],
) -> Result<(PlayerShip, AstroObject), String> {
    if g_info.player_ship.get_torpedoes() <= 0 {
        return Err("No torpedoes available!  You need to dock at a Starbase.".to_string());
    }

    let res = crate::manifest::create_qi_enemy_vec(g_info)?;
    //   if let Err(e) = res {
    //       return Err(e);
    //   }

    let potential_bad_guys = res; //.unwrap().1;
    let mut tgt_sector: AstroObject;
    let found_it: bool;

    if cmd_vector.len() == 1 {
        // look for a Klingon
        let res_tuple = crate::manifest::calc_distance_to_enemy(
            g_info,
            potential_bad_guys.0.clone(),
            AstroType::Klingon,
        );
        found_it = res_tuple.0;

        tgt_sector = res_tuple.2;

        if !found_it {
            //now look for Romulan
            let res_tuple = crate::manifest::calc_distance_to_enemy(
                g_info,
                potential_bad_guys.0,
                AstroType::Romulan,
            );
            tgt_sector = res_tuple.2;
        }
        println!("targetting {:?} with torpedoe", tgt_sector);
        tgt_sector.kill_enemy();
        let mut updated_enterprise = g_info.player_ship;
        updated_enterprise.use_torpedoe();
        Ok((updated_enterprise, tgt_sector))
    } else if cmd_vector.len() == 3 {
        let res = crate::ui::validate_x_y_input(cmd_vector, MAX_SECTOR_SIZE_I8);
        let mut n_info: AstroObject;
        match res {
            Ok(_) => {
                let tgt_sector: (i8, i8) = res.unwrap();
                let mut pbg = potential_bad_guys.0.clone();
                for si in pbg.iter_mut() {
                    n_info = *si;
                    if n_info.is_same_sect_tuple(tgt_sector) {
                        println!(
                            "Killed the {:?} at {:?} with torpedoe",
                            n_info.get_astro_type(),
                            tgt_sector
                        );
                        n_info.kill_enemy(); //potential_bad_guys[0].kill_enemy();
                        let mut updated_enterprise = g_info.player_ship;
                        updated_enterprise.use_torpedoe();
                        return Ok((updated_enterprise, n_info));
                    }
                }
                Err("No enemy found at specified sector".to_string())
            }
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
    g_info: &Manifest,
    cmd_vector: &[String],
) -> Result<(PlayerShip, AstroObject), String> {
    if g_info.player_ship.get_energy() <= 0 {
        return Err(
            "No energy is available to be used.  You need to dock at a Starbase.".to_string(),
        );
    }
    let res = crate::manifest::create_qi_enemy_vec(g_info)?;
    let potential_bad_guys = res;
    let mut tgt_sector: AstroObject;
    let found_it: bool;
    let mut current_distance: f64;//=1000.0;

    if cmd_vector.len() == 1 {
        // look for a Romulan
        let res_tuple = crate::manifest::calc_distance_to_enemy(
            g_info,
            potential_bad_guys.0.clone(),
            AstroType::Romulan,
        );
        found_it = res_tuple.0;
        current_distance = res_tuple.1;
        tgt_sector = res_tuple.2;

        if !found_it {
            //now look for Klingon
            let res_tuple = crate::manifest::calc_distance_to_enemy(
                g_info,
                potential_bad_guys.0.clone(),
                AstroType::Klingon,
            );
            //found_it = res_tuple.0;
            current_distance = res_tuple.1;
            tgt_sector = res_tuple.2;
        }
        // determine_enemy_attack(g_info);
        println!("targetting {:?} with phaser", tgt_sector);
        tgt_sector.kill_enemy();

        let mut updated_player_ship = g_info.player_ship;
        updated_player_ship.use_energy(1000 + (current_distance * 100.0) as isize);
        Ok((updated_player_ship, tgt_sector))
    } else if cmd_vector.len() == 3 {
        let res = crate::ui::validate_x_y_input(cmd_vector, MAX_SECTOR_SIZE_I8);
        let mut n_info: AstroObject;
        match res {
            Ok(_) => {
                let tgt_sector: (i8, i8) = res.unwrap();
                let mut pbg = potential_bad_guys.0.clone();
                for si in pbg.iter_mut() {
                    n_info = *si;
                    if n_info.is_same_sect_tuple(tgt_sector) {
                        println!(
                            "Killed the {:?} at {:?} with phaser",
                            n_info.get_astro_type(),
                            tgt_sector
                        );
                        n_info.kill_enemy();
                        current_distance = 4.0;
                        let mut updated_enterprise = g_info.player_ship;
                        updated_enterprise.use_energy(1000 + (current_distance * 100.0) as isize);
                        return Ok((updated_enterprise, n_info));
                    }
                }
                Err("No enemy found at specified sector".to_string())
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        Err("Command format should be  pha x y".to_string())
    }
}
/*/
pub fn determine_enemy_attack(g_info: &Manifest) -> Result<i32,String> {
    let mut n_vec: Vec<AstroObject> = Vec::new();

    let qi_vec = g_info.create_quadrant_vec(g_info.player_ship.get_entity());

    let potential_bad_guys =
        crate::manifest::create_bad_guy_qi_vec(&qi_vec, g_info.player_ship.get_entity(), false);
    if potential_bad_guys.is_empty() {
        return Err("No enemy targets found in the current quadrant.".to_string());
    }

    let mut current_distance: f64;
    for si in qi_vec.iter() {
        let n_info = *si;
        //println!("deter enemy attack {:?}", n_info);
        let bad_guy_type: AstroType = n_info.get_astro_type();
        if bad_guy_type == AstroType::Klingon || bad_guy_type == AstroType::Romulan {
//            if chk_path {
                let path_res = crate::manifest::is_straight_line_path_clear(&qi_vec, g_info.player_ship.get_entity(), n_info);
                if path_res.is_ok() && path_res.unwrap() {
                    current_distance = g_info.player_ship.get_entity().calc_sector_distance(n_info);
                    //println!(" {}  {:?}",current_distance,n_info);
                    n_vec.push(n_info);
                }
//            } else {
//                n_vec.push(n_info);
//            }
        }
    }
    //println!("potential targets {:?}",n_vec);
    
    //for
    
    
    
    //n_vec

    let res = crate::manifest::create_qi_enemy_vec(g_info)?;
    let potential_bad_guys = res;
    //let mut tgt_sector: AstroObject;
    let found_it: bool;
    let mut current_distance: f64;

    let res_tuple = crate::manifest::calc_distance_to_enemy(
        g_info,
        potential_bad_guys.0.clone(),
        AstroType::Romulan,
    );
    found_it = res_tuple.0;
    current_distance = res_tuple.1;
    let mut enemy_sector = res_tuple.2;

    Ok(3)
   // potential_bad_guys.0
} */
