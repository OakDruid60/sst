#![warn(missing_docs)]
//! # help.rs
//!
//! This is stuff related to displaying general user help information.
//!

use crate::manifest::Manifest;
use crate::ui::{BORDER_HORZ_60, BORDER_LL, BORDER_LR, BORDER_VERT, COLOR_RESET, BORDER_COLOR_GREEN};

// ===========================================================
/// # help_screen
///
/// Put the game instructions on the console.
///
/// **Note** needs more details.
///

pub fn help_screen(g_info: &Manifest) {
    let bc = BORDER_COLOR_GREEN; //crate::helpers::alert_status_of_quadrant2(&qi_vec);
    crate::ui::disp_title("Command Help", g_info, bc);
    let bv: &str = &(bc.to_owned() + BORDER_VERT + COLOR_RESET);
    println!("  {bv} {:<58} {bv}", "qui - quit");
    println!("  {bv} {:<58} {bv}", " ");
    println!("  {bv} {:<58} {bv}", "lrs - long range sensor scan");
    println!("  {bv} {:<58} {bv}", "srs - short range sensor scan");
    println!("  {bv} {:<58} {bv}", "pha - fire phasers");
    println!("  {bv} {:<58} {bv}", "tor - fire torpedoe");
    println!("  {bv} {:<58} {bv}", "jum - jump to new quadrant");
    println!(
        "  {bv} {:<58} {bv}",
        "mov - move to new sector in current quadrant"
    );
    println!("  {bv} {:<58} {bv}", " ");
    println!("  {bv} {:<58} {bv}", "save - save");
    println!("  {bv} {:<58} {bv}", "rest - restore");
    println!("  {bv} {:<58} {bv}", " ");
    println!("  {bv} {:<58} {bv}", "sta - stats");
    println!("  {bv} {:<58} {bv}", "hel - help");
    println!("  {bc}{BORDER_LL}{BORDER_HORZ_60}{BORDER_LR}{COLOR_RESET}");
}
