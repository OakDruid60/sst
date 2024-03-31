#![warn(missing_docs)]
//! # weapon.rs
//!
//! This is stuff related to weapons of the Enterprise.

use crate::enterprise::ShipInfo;
use crate::manifest::constants::MAX_SECTOR_SIZE_I8;
use crate::manifest::entity::Entity;
use crate::manifest::enums::SectorType;
use crate::manifest::Manifest;

// =================================================================
/// # fire_torpedoe
///
///
pub fn fire_torpedoe(
    g_info: &Manifest,
    cmd_vector: &Vec<String>,
) -> Result<(ShipInfo, Entity), String> {
    if g_info.enterprise.get_torpedoes() <= 0 {
        return Err("No torpedoes available!  You need to dock at a Starbase.".to_string());
    }

    let res = create_qi_enemy_vec(g_info);
    match res {
        Err(e) => {
            return Err(e);
        }
        Ok(_) => {}
    }

    let potential_bad_guys = res.unwrap().1;
    let mut tgt_sector: Entity;
    let found_it: bool;

    if cmd_vector.len() == 1 {
        // look for a Klingon
        let res_tuple =
            calc_distance_to_enemy(g_info, potential_bad_guys.clone(), SectorType::Klingon);
        found_it = res_tuple.0;

        tgt_sector = res_tuple.2;

        if !found_it {
            //now look for Romulan
            let res_tuple =
                calc_distance_to_enemy(g_info, potential_bad_guys.clone(), SectorType::Romulan);
            tgt_sector = res_tuple.2;
        }
        println!("targetting {:?} with torpedoe", tgt_sector);
        tgt_sector.kill_enemy();
        let mut updated_enterprise = g_info.enterprise;
        updated_enterprise.use_torpedoe();
        return Ok((updated_enterprise, tgt_sector));
    } else if cmd_vector.len() == 3 {
        let res = crate::helpers::validate_x_y_input(&cmd_vector, MAX_SECTOR_SIZE_I8);
        let mut n_info: Entity;
        match res {
            Ok(_) => {
                let tgt_sector: (i8, i8) = res.unwrap();
                let mut pbg = potential_bad_guys.clone();
                for si in pbg.iter_mut() {
                    n_info = *si;
                    if n_info.is_same_sect_tuple(tgt_sector) {
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
    cmd_vector: &Vec<String>,
) -> Result<(ShipInfo, Entity), String> {
    if g_info.enterprise.get_energy() <= 0 {
        return Err(
            "No energy is available to be used.  You need to dock at a Starbase.".to_string(),
        );
    }
    let res = create_qi_enemy_vec(g_info);
    match res {
        Err(e) => {
            return Err(e);
        }
        Ok(_) => {}
    }

    let potential_bad_guys = res.unwrap().1;
    let mut tgt_sector: Entity;
    let found_it: bool;
    let mut current_distance: f64;

    if cmd_vector.len() == 1 {
        // look for a Romulan
        let res_tuple =
            calc_distance_to_enemy(g_info, potential_bad_guys.clone(), SectorType::Romulan);
        found_it = res_tuple.0;
        current_distance = res_tuple.1;
        tgt_sector = res_tuple.2;

        if !found_it {
            //now look for Klingon
            let res_tuple =
                calc_distance_to_enemy(g_info, potential_bad_guys.clone(), SectorType::Klingon);
            //found_it = res_tuple.0;
            current_distance = res_tuple.1;
            tgt_sector = res_tuple.2;
        }
        println!("targetting {:?} with phaser", tgt_sector);
        tgt_sector.kill_enemy();

        let mut updated_enterprise = g_info.enterprise;
        updated_enterprise.use_energy(1000 + (current_distance * 100.0) as isize);
        return Ok((updated_enterprise, tgt_sector));
    } else if cmd_vector.len() == 3 {
        let res = crate::helpers::validate_x_y_input(&cmd_vector, MAX_SECTOR_SIZE_I8);
        let mut n_info: Entity;
        match res {
            Ok(_) => {
                let tgt_sector: (i8, i8) = res.unwrap();
                let mut pbg = potential_bad_guys.clone();
                for si in pbg.iter_mut() {
                    n_info = *si;
                    if n_info.is_same_sect_tuple(tgt_sector) {
                        println!(
                            "Killed the {:?} at {:?} with phaser",
                            n_info.get_sector_type(),
                            tgt_sector
                        );
                        n_info.kill_enemy();
                        current_distance = 4.0;
                        let mut updated_enterprise = g_info.enterprise;
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

// =================================================================
/// # create_qi_enemy_vec
///
///
fn create_qi_enemy_vec(g_info: &Manifest) -> Result<(Vec<Entity>, Vec<Entity>), String> {
    let qi_vec = g_info.create_quadrant_vec(g_info.enterprise.get_entity());
    let potential_bad_guys =
        crate::manifest::create_bad_guy_qi_vec(&qi_vec, g_info.enterprise.get_entity(), false);
    if potential_bad_guys.len() == 0 {
        return Err(format!("No enemy targets found in the current quadrant.").to_string());
    }
    let potential_bad_guys =
        crate::manifest::create_bad_guy_qi_vec(&qi_vec, g_info.enterprise.get_entity(), true);
    if potential_bad_guys.len() == 0 {
        return Err(format!("No enemy targets found in current quadrant along a straight line path from the Enterprise to the target that is not blocked by some other object.").to_string());
    }
    Ok((qi_vec.clone(), potential_bad_guys.clone()))
}

// =================================================================
/// # calc_distance_to_enemy
///
///
fn calc_distance_to_enemy(
    g_info: &Manifest,
    potential_bad_guys: Vec<Entity>,
    enemy_type: SectorType,
) -> (bool, f64, Entity) {
    let mut n_info: Entity;
    let mut found_it = false;
    let mut potential_enemies = potential_bad_guys.clone();
    let mut tgt_sector = potential_enemies[0].clone();
    let mut current_distance: f64 = 1000.0;
    for si in potential_enemies.iter_mut() {
        n_info = *si;
        if n_info.get_sector_type() == enemy_type {
            if !found_it {
                found_it = true;
                tgt_sector = n_info.clone();
                current_distance = tgt_sector.calc_sector_distance(g_info.enterprise.get_entity());
            } else {
                let n_tgt_sector = n_info.clone();
                let new_distance: f64 =
                    n_tgt_sector.calc_sector_distance(g_info.enterprise.get_entity());
                if new_distance < current_distance {
                    current_distance = new_distance;
                    tgt_sector = n_tgt_sector;
                }
            }
        }
    }
    return (found_it, current_distance, tgt_sector.clone());
}
