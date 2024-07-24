#![warn(missing_docs)]
//! # enums.rs
// =====================================================================
/// #shipsystem
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ShipSystem {
    t: SystemType,
    dmg_sys: i32,
    cur_sys: i32,
    replenish: i32,
}

// =============================
// =============================
/// # Damage
///
pub struct Damage {
    typ: DamageType,
    amount: i32,
}
