#![warn(missing_docs)]
//! # movement.rs
//!
//! Functions for moving around the Enterprise
//!

use crate::astro::{AstroObject, AstroType};
use crate::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
use crate::manifest::Manifest;
use crate::ship_info::PlayerShip;

// =================================================================
/// # jump_enterprise
///
/// jump the enterprise to a new quadrant  It is possible to jump to the starbase.
pub fn jump_enterprise(g_info: &Manifest, cmd_vector: &[String]) -> Result<PlayerShip, String> {
    if cmd_vector.len() == 2 {
        if cmd_vector[1].starts_with("sb") {
            //let g_tmp = g_info.clone();
            let tqi_vec = crate::manifest::isolate_type(g_info, AstroType::Starbase);
            //println!("{:?}", tqi_vec);
            let star_base_vec = crate::manifest::create_vec_of_type(&tqi_vec, AstroType::Starbase);
            //println!("{:?}", star_base_vec);
            let mut updated_player_ship = g_info.player_ship;
            let sector_bounds = star_base_vec[0].calc_nearby_sector_bounds();

            let dock_quad = star_base_vec[0].ret_quad_tuple();
            let dock_sect = sector_bounds.0;
            let dock_loc = AstroObject::create(
                (0, 0, dock_quad.0, dock_quad.1, dock_sect.0, dock_sect.1),
                AstroType::PlayerShip,
            );

            println!("Docking player ship at {:?}", dock_loc);
            updated_player_ship.set_entity(dock_loc);
            updated_player_ship.reset_torpedoes();
            updated_player_ship.reset_energy();
            Ok(updated_player_ship)
        } else {
            Err("Command format should be  jum x y".to_string())
        }
    } else if cmd_vector.len() == 3 || cmd_vector.len() == 4 {
        let res = crate::ui::validate_x_y_input(cmd_vector, MAX_GALAXY_SIZE_I8);

        match res {
            Ok(_) => {
                let tgt_quad = res.unwrap();
                let mut cmd_mode: String = "random".to_string();
                if cmd_vector.len() == 4 {
                    cmd_mode = cmd_vector[3].clone();
                    //println!("{}", cmd_mode);
                }
                println!(
                    "jumping player ship to {:?} sector in quadrant {:?}",
                    cmd_mode, tgt_quad
                );

                let new_ship_loc = AstroObject::create_ship_astro_object(tgt_quad, cmd_mode);
                let distance = g_info
                    .player_ship
                    .get_entity()
                    .calc_quad_distance(new_ship_loc) as isize;

                let mut updated_player_ship = g_info.player_ship;
                updated_player_ship.set_entity(new_ship_loc);
                if cmd_vector.len() == 3 {
                    updated_player_ship.use_energy(distance * 1000);
                }

                Ok(updated_player_ship)
            }
            //
            Err(e) => Err(e),
        }
    } else {
        Err("Command has the wrong number of arguments. Format should be  jum x y".to_string())
    }
}

// =================================================================
/// # move_enterprise
///
/// Move the enterprise to a new sector in the current quadrant
pub fn move_enterprise(g_info: &Manifest, cmd_vector: &[String]) -> Result<PlayerShip, String> {
    if cmd_vector.len() == 3 {
        let res = crate::ui::validate_x_y_input(cmd_vector, MAX_SECTOR_SIZE_I8);

        match res {
            Ok(_) => {
                let new_sect = res.unwrap();
                let loc_tmp = g_info.player_ship.get_entity();
                if loc_tmp.is_same_sect_tuple(new_sect) {
                    return Err(format!(
                        "Target destination {:?} is where the player ship currently is.",
                        new_sect
                    ));
                }
                let new_quad = loc_tmp.ret_quad_tuple();
                let new_enterprise_loc = AstroObject::create(
                    (0, 0, new_quad.0, new_quad.1, new_sect.0, new_sect.1),
                    AstroType::PlayerShip,
                );
                println!(
                    " new player ship loc {:?}",
                    new_enterprise_loc.to_compact_string()
                );
                let qi_vec = g_info.create_quadrant_vec(loc_tmp);
                let tgt_info = crate::manifest::find_actual_sector_info(&qi_vec, new_sect);
                if tgt_info.get_astro_type() != AstroType::Empty
                    && tgt_info.get_astro_type() != AstroType::KilledKlingon
                    && tgt_info.get_astro_type() != AstroType::KilledRomulan
                {
                    return Err(format!(
                        "Target destination {:?} is not empty ({:?}).",
                        new_sect,
                        tgt_info.get_astro_type()
                    ));
                }
                println!("Moving player ship to sector {:?}", new_sect);

                let distance = g_info
                    .player_ship
                    .get_entity()
                    .calc_sector_distance(new_enterprise_loc)
                    as isize;

                let mut updated_player_ship = g_info.player_ship;
                updated_player_ship.set_entity(new_enterprise_loc);
                updated_player_ship.use_energy(distance * 10);
                Ok(updated_player_ship)
            }
            //
            Err(e) => Err(e),
        }
    } else {
        Err("Command format should be  mov x y".to_string())
    }
}
