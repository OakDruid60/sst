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
        self(t: General, dmg_sys: 0.0, cur_sys: 0.0,
             rpl_sys: 0.0)
    }

    pub fn consume(&self, amount: f32) {
    }

    pub fn replenish(&self) {
    }

    pub fn is_operational(&self) -> bool {        
    }
}



// =============================
// =============================
/// # SystemType
///
#[derive(Serialize, Deserialize)]
pub enum SystemType {
    General,
    ImpulseEng,
    Phaser,
    Torpedoe,
    WarpEng,

}
