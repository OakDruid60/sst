#![warn(missing_docs)]
//! # command.rs
//!
//! User Command functions

use crate::enums::CmdType;

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
