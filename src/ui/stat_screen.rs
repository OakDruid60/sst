#![warn(missing_docs)]
//! # stat_screen.rs
//!
//! This is stuff related to displaying information, etc.
//!

use crate::manifest::Manifest;
use crate::ui::{BORDER_HORZ_60, BORDER_LL, BORDER_LR, BORDER_VERT, COLOR_RESET};

// ===========================================================
/// # game_stat_disp
///
pub fn game_stat_disp(g_info: &Manifest) {
    let human_output: Vec<String> = g_info.clone().game_stat_create_disp_vec(true);

    let qi_vec = crate::manifest::isolate_cur_quadrant(g_info);
    let bc = crate::ui::alert_status_of_quadrant2(&qi_vec);

    crate::ui::disp_title("Game Stats", g_info, bc);

    for element in &human_output {
        println!(
            "  {bc}{BORDER_VERT}{COLOR_RESET}{} {: <20}  {bc}{BORDER_VERT}{COLOR_RESET}",
            element, " "
        );
    }

    println!("  {bc}{BORDER_LL}{BORDER_HORZ_60}{BORDER_LR}{COLOR_RESET}");
}
