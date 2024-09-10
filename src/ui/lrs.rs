#![warn(missing_docs)]
//! # lrs.rs
//!
//! This is stuff related to displaying the long range sensor scan.

use crate::astro::{AstroObject, AstroType};
use crate::manifest::constants::MAX_GALAXY_SIZE_I8;
use crate::manifest::Manifest;
use crate::ui::{BORDER_HORZ_60, BORDER_LL, BORDER_LR, BORDER_VERT, COLOR_RESET};

// ==========================================================
/// # long_range_sensor_disp
///
/// Put the Long Range sensor scan on the console.  Checks each quadrant
/// to see if it has been charted.  The information is encoded in a 5 character
/// string that is colorized for emphasis.
///
/// **Note** The sectors in the vicinity of the ENTERPRISE are marked as
/// charted even if they have not been.   Also the starbase quadrant is
/// also marked as charted.
///
pub fn long_range_sensor_disp(g_info: &Manifest) {
    //
    let qi_vec = crate::manifest::isolate_cur_quadrant(&g_info);
    let bc = crate::ui::alert_status_of_quadrant2(&qi_vec);
    crate::ui::disp_title("Long Range Scan", g_info, bc);

    for yy in { 00..MAX_GALAXY_SIZE_I8 }.rev() {
        let mut row_string = format!("{} {bc}{BORDER_VERT}{COLOR_RESET}", yy);
        for xx in 0..MAX_GALAXY_SIZE_I8 {
            let tmp_quadx: i8 = xx as i8;
            let tmp_quady: i8 = yy as i8;
            let zero: i8 = 0 as i8;
            let tmp_loc =
                AstroObject::create((0, 0, tmp_quadx, tmp_quady, zero, zero), AstroType::Empty);

            let tmp_qi_vec = g_info.create_quadrant_vec(tmp_loc);
            let mut tmp: String = crate::manifest::compact_summary_string(&tmp_qi_vec);
            //if g_info.charted[xx as usize][yy as usize] {
            //    tmp = format!("{}", tmp);
            //} else {
            //let  tmp = format!("   .   ");
            //}
            row_string.push_str(tmp.as_str());
        }
        row_string.push_str(format!("    {bc}{BORDER_VERT}{COLOR_RESET}").as_str());
        if yy == 0 {
            row_string.push_str(
                format!("\n  {bc}{BORDER_LL}{BORDER_HORZ_60}{BORDER_LR}{COLOR_RESET}").as_str(),
            );
            row_string.push_str(
                format!(
                    "\n   {:^7}{:^7}{:^7}{:^7}{:^7}{:^7}{:^7}{:^7}\n",
                    0, 1, 2, 3, 4, 5, 6, 7
                )
                .as_str(),
            );
        } else {
            row_string.push('\n');
        }

        print!("{}", row_string);
    }
}
