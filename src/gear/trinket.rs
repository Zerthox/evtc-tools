use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by a Trinket.
///
/// This only the base base stats shared across all trinkets.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    IntoPrimitive,
    TryFromPrimitive,
)]
#[repr(u32)]
pub enum Trinket {
    /// Used by Legendary Back, Amulet, Accessory, Ring and Ascended Accessory.
    Berserker = 15742,

    /// Used by Ascended Ring.
    ///
    /// Has +5 Agony Resistance in the tooltip, but does not grant it.
    BerserkerAlt = 15755,
}
