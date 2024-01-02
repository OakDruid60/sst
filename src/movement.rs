#![warn(missing_docs)]
//! # movement
//!
//! Functions for moving around the Enterprise
use crate::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::enums::SectorType;
use crate::gamedata::GameWideData;
use crate::location::Location;
use crate::ship::ShipInfo;

// =================================================================
/// # jump_enterprise
///
/// jump the enterprise to a new quadrant  It is possible to jump to the starbase.
pub fn jump_enterprise(
    g_info: &GameWideData,
    cmd_vector: &Vec<String>,
) -> Result<ShipInfo, String> {
    if cmd_vector.len() == 2 {
        if cmd_vector[1].starts_with("sb") {
            let g_tmp = g_info.clone();
            let star_base_vec =
                crate::gamedata::create_vec_of_type(&g_tmp.gal_vec, SectorType::Starbase);

            println!("locating starbase at {:?}", star_base_vec[0].get_location());
            let mut updated_enterprise = g_info.enterprise;
            let sector_bounds = star_base_vec[0].get_location().calc_nearby_sector_bounds();
            // println!("{:?}",sector_bounds);

            let dock_quad = star_base_vec[0].get_location().gen_quad_tuple();
            let dock_sect = sector_bounds.0;
            let dock_loc = Location::create((dock_quad.0, dock_quad.1, dock_sect.0, dock_sect.1));

            println!("Docking Enterprise at {:?}", dock_loc);
            updated_enterprise.set_location(dock_loc);
            updated_enterprise.reset_torpedoes();
            updated_enterprise.reset_energy();
            Ok(updated_enterprise)
        } else {
            Err("Command format should be  jum x y".to_string())
        }
    } else if cmd_vector.len() == 3 {
        let res = crate::helpers::validate_x_y_input(&cmd_vector, MAX_GALAXY_SIZE_I8);

        match res {
            Ok(_) => {
                let tgt_quad = res.unwrap();
                println!(
                    "jumping Enterprise to random sector in quadrant {:?}",
                    tgt_quad
                );

                let new_enterprise_loc = crate::location::create_random_loc_given_quad(tgt_quad);
                let distance = crate::location::calc_quad_distance(
                    g_info.enterprise.get_location(),
                    new_enterprise_loc,
                ) as isize;

                let mut updated_enterprise = g_info.enterprise;
                updated_enterprise.set_location(new_enterprise_loc);
                updated_enterprise.use_energy(distance * 1000);

                Ok(updated_enterprise)
            }
            //
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        Err("Command format should be  jum x y".to_string())
    }
}

// =================================================================
/// # move_enterprise
///
/// Move the enterprise to a new sector in the current quadrant
pub fn move_enterprise(
    g_info: &GameWideData,
    cmd_vector: &Vec<String>,
) -> Result<ShipInfo, String> {
    if cmd_vector.len() == 3 {
        let res = crate::helpers::validate_x_y_input(&cmd_vector, MAX_SECTOR_SIZE_I8);

        match res {
            Ok(_) => {
                let new_sect = res.unwrap();
                let loc_tmp = g_info.enterprise.get_location().clone();
                let new_quad = loc_tmp.gen_quad_tuple();
                let new_enterprise_loc =
                    Location::create((new_quad.0, new_quad.1, new_sect.0, new_sect.1));
                let qi_vec = g_info.create_quadrant_vec(loc_tmp);
                let tgt_info = crate::gamedata::find_actual_sector_info(&qi_vec, new_sect);
                println!("at {:?}  {:?} ", new_sect, tgt_info.get_sector_type());
                if tgt_info.get_sector_type() != SectorType::Empty
                    && tgt_info.get_sector_type() != SectorType::KilledKlingon
                    && tgt_info.get_sector_type() != SectorType::KilledRomulan
                {
                    return Err(format!(
                        "Target destination {:?} is not empty({:?}).",
                        new_sect,
                        tgt_info.get_sector_type()
                    ));
                }
                println!("Moving Enterprise to sector {:?}", new_sect);

                let distance = crate::location::calc_sector_distance(
                    g_info.enterprise.get_location(),
                    new_enterprise_loc,
                ) as isize;

                let mut updated_enterprise = g_info.enterprise;
                updated_enterprise.set_location(new_enterprise_loc);
                updated_enterprise.use_energy(distance * 10);
                Ok(updated_enterprise)
            }
            //
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        Err("Command format should be  mov x y".to_string())
    }
}
