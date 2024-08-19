#![warn(missing_docs)]
//! # ui.rs
//!
//! This is stuff related to the user interface (ui)

/*
//use serde_json::to_string;
use colored::Colorize;
use std::io::{stdin, stdout, Write};

pub mod help_screen;
pub mod logo_screen;
pub mod lrs;
pub mod stat_screen;
pub mod srs;

//use crate::enterprise::ShipInfo;
use crate::manifest::constants::MAX_GALAXY_SIZE_I8;
//use crate::manifest::enums::{CmdType, EntityType};
use crate::manifest::{Manifest, Galaxy};

pub const BORDER_COLOR_RED: &str = "\x1b[91m";
pub const BORDER_COLOR_YELLOW: &str = "\x1b[93m";
pub const BORDER_COLOR_GREEN: &str = "\x1b[92m";
pub const BORDER_COLOR_CYAN: &str = "\x1b[96m";
//pub const BORDER_COLOR: &str = "\x1b[91m";
pub const BORDER_HORZ_60: &str = "════════════════════════════════════════════════════════════";
pub const BORDER_VERT: &str = "║";
pub const BORDER_UR: &str = "╗";
pub const BORDER_UL: &str = "╔";
pub const BORDER_LR: &str = "╝";
pub const BORDER_LL: &str = "╚";
pub const BORDER_MR: &str = "╣";
pub const BORDER_ML: &str = "╠";

pub const COLOR_RESET: &str = "\x1b[0m";


// ==========================================================
/// # disp_title
///
/// New attempt at a command title bar
pub fn disp_title(title: &str, g_info: &Manifest, bc: &str) {
    //let tmp_title = format!("{}", title.underline())
    println!("");
    //let BORDER_COL: &str  = BORDER_COLOR_GREEN;
    println!("  {bc}{BORDER_UL}{BORDER_HORZ_60}{BORDER_UR}{COLOR_RESET}");
    println!(
        "  {bc}{BORDER_VERT}{COLOR_RESET} {: <30}{: >28} {bc}{BORDER_VERT}{COLOR_RESET}",
        title,
        g_info.enterprise.get_entity().to_compact_string()
    );
    println!("  {bc}{BORDER_ML}{BORDER_HORZ_60}{BORDER_MR}{COLOR_RESET}");
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

// ==========================================================
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
    } else if cmd_string.starts_with("q") || cmd_string.starts_with("exit") {
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
*/