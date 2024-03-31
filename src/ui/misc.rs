#![warn(missing_docs)]
//! # misc.rs
//!
//! This is stuff related to displaying information, the sensor scans, the logo, etc.
//!┏━ ━ ┓┃
use crate::manifest::Manifest;

// ==========================================================================
/// # game_stat_disp
///
pub fn game_stat_disp(g_info: &Manifest) {
    let human_output: Vec<String> = g_info.clone().game_stat_create_disp_vec(true);

    crate::ui::disp_title("Game Stats", g_info);
    for element in &human_output {
        println!("  ┃{} {: <20} ┃ ", element, " ");
    }
    println!("  ┗{:━^63}┛", "━");
}

// ==========================================================================
/// # help_screen
///
/// Put the game instructions on the console.
///
/// **Note** needs more details.
///
pub fn help_screen(g_info: &Manifest) {
    crate::ui::disp_title("Command Help", g_info);

    println!("  ┃ {:<60}  ┃", "qui - quit");
    println!("  ┃ {:<60}  ┃", " ");
    println!("  ┃ {:<60}  ┃", "lrs - long range sensor scan");
    println!("  ┃ {:<60}  ┃", "srs - short range sensor scan");
    println!("  ┃ {:<60}  ┃", "pha - fire phasers");
    println!("  ┃ {:<60}  ┃", "tor - fire torpedoe");
    println!("  ┃ {:<60}  ┃", "jum - jump to new quadrant");
    println!(
        "  ┃ {:<60}  ┃",
        "mov - move to new sector in current quadrant"
    );
    println!("  ┃ {:<60}  ┃", " ");
    println!("  ┃ {:<60}  ┃", "save - save");
    println!("  ┃ {:<60}  ┃", "rest - restore");
    println!("  ┃ {:<60}  ┃", " ");
    println!("  ┃ {:<60}  ┃", "sta - stats");
    println!("  ┃ {:<60}  ┃", "hel - help");
    println!("  ┗{:━^63}┛", "━");
}
