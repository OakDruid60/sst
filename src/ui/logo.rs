#![warn(missing_docs)]
//! # logo.rs
//!
//! This is stuff related to displaying the logo.

//use crate::constants::{MAX_GALAXY_SIZE_I8, MAX_SECTOR_SIZE_I8};
//use crate::enums::SectorType;
//use crate::gamedata::{Entity, GameWideData};

use colored::*;

// ==========================================================================
/// # game_logo
///
/// put the opening game identification info on the console
///
pub fn game_logo() {
    println!("");
    println!(
        "{} v{} -- {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    println!("");

    println!("                     ,------*------,");
    println!("     ,-------------   '---  ------'");
    println!("      '-------- --'      / /");
    println!("          ,---' '-------/ /--,");
    println!("           '----------------'");
    println!("");

    println!(
        "{} {} {} {} \n",
        "     ",
        "USS ENTERPRISE".green(),
        " --- ".clear(),
        "NCC-1701".bright_purple()
    );
}
