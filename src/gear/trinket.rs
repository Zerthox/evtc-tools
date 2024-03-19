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
    #[serde(rename = "Berserker's Trinket")]
    Berserkers = 15742,

    /// Used by Ascended Ring.
    ///
    /// Has +5 Agony Resistance in the tooltip, but does not grant it.
    #[serde(rename = "Berserker's Trinket")]
    BerserkersAlt = 15755,

    #[serde(rename = "Assassin's Trinket")]
    Assassins = 32045,

    #[serde(rename = "Celestial Trinket")]
    Celestial = 15750,

    #[serde(rename = "Diviner's Trinket")]
    Diviner = 53915,

    #[serde(rename = "Dragon's Trinket")]
    Dragons = 63926,

    #[serde(rename = "Giver's Trinket")]
    Givers = 48934,

    #[serde(rename = "Grieving Trinket")]
    Grieving = 41338,

    #[serde(rename = "Harrier's Trinket")]
    Harriers = 44930,

    #[serde(rename = "Magi's Trinket")]
    Magis = 25442,

    #[serde(rename = "Minstrel's Trinket")]
    Minstrels = 33290,

    #[serde(rename = "Rampager's Trinket")]
    Rampagers = 17071,

    #[serde(rename = "Ritualist's Trinket")]
    Ritualists = 64839,

    #[serde(rename = "Sinister Trinket")]
    Sinister = 2566,

    #[serde(rename = "Trailblazer's Trinket")]
    Trailblazers = 33199,

    #[serde(rename = "Viper's Trinket")]
    Vipers = 32391,
}
