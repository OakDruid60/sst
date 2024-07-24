#![warn(missing_docs)]
use serde::{Deserialize, Serialize};
//! # sys.rs
// =====================================================================
/// #ShipSystem
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ShipSystem {
    t: SystemType,
    dmg_sys: f32,
    cur_sys: f32,
    rpl_sys: f32,
}

impl ShipSystem {
    pub fn new() -> Self {
    }

    pub fn consume(&self, amount: i32) {
    }

    pub fn replenish(&self) {
    }

    pub fn is_damaged(&self) -> bool {
        
    }
}



// =============================
// =============================
/// # SystemType
///
#[derive(Serialize, Deserialize)]
pub enum SystemType {
    ImpulseEng,
    Phaser,
    Torpedoe,
    WarpEng,

}
