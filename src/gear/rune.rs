use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by a 6 piece Rune.
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
pub enum Rune {
    #[serde(rename = "Rune of the Afflicted")]
    Afflicted = 9585,

    #[serde(rename = "Rune of Balthazar")]
    Balthazar = 53263,

    #[serde(rename = "Rune of the Berserker")]
    Berserker = 33124,

    #[serde(rename = "Rune of the Eagle")]
    Eagle = 9483,

    #[serde(rename = "Rune of the Elementalist")]
    Elementalist = 9615,

    #[serde(rename = "Rune of the Firebrand")]
    Firebrand = 41177,

    #[serde(rename = "Rune of the Flame Legion")]
    FlameLegion = 9617,

    #[serde(rename = "Rune of the Golemancer")]
    Golemancer = 9654,

    #[serde(rename = "Rune of the Krait")]
    Krait = 24178,

    #[serde(rename = "Rune of Leadership")]
    Leadership = 31943,

    #[serde(rename = "Rune of the Lich")]
    Lich = 9590,

    #[serde(rename = "Rune of the Mad King")]
    MadKing = 15264,

    #[serde(rename = "Rune of Mercy")]
    Mercy = 9494,

    #[serde(rename = "Rune of the Mesmer")]
    Mesmer = 9646,

    #[serde(rename = "Rune of the Monk")]
    Monk = 24125,

    #[serde(rename = "Rune of the Nightmare")]
    Nightmare = 35543,

    #[serde(rename = "Rune of the Ogre")]
    Ogre = 53349,

    #[serde(rename = "Rune of the Pack")]
    Pack = 24253,

    #[serde(rename = "Rune of Perplexity")]
    Perplexity = 53357,

    #[serde(rename = "Rune of Rebirth")]
    Rebirth = 41444,

    #[serde(rename = "Rune of the Rebirth (Active)")]
    RebirthActive = 39944,

    #[serde(rename = "Rune of the Renegade")]
    Renegade = 40695,

    #[serde(rename = "Rune of Sanctuary")]
    Sanctuary = 9645,

    #[serde(rename = "Rune of the Scholar")]
    Scholar = 9620,

    #[serde(rename = "Rune of the Spellbreaker")]
    Spellbreaker = 42674,

    #[serde(rename = "Rune of Strength")]
    Strength = 9582,

    #[serde(rename = "Rune of the Tempest")]
    Tempest = 31944,

    #[serde(rename = "Rune of the Thief")]
    Thief = 9638,

    #[serde(rename = "Rune of the Undead")]
    Undead = 9711,

    #[serde(rename = "Rune of Water")]
    Water = 9598,

    #[serde(rename = "Rune of Thorns")]
    Thorns = 32456,

    #[serde(rename = "Rune of Tormenting")]
    Tormenting = 20479,
}
