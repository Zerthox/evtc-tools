use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by a Relic.
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
pub enum Relic {
    #[serde(rename = "Relic of Akeem")]
    Akeem = 70452,

    #[serde(rename = "Relic of Fireworks")]
    Fireworks = 71259,

    #[serde(rename = "Relic of the Aristocracy")]
    Aristocracy = 71257,

    #[serde(rename = "Relic of the Dragonhunter")]
    Dragonhunter = 70680,

    #[serde(rename = "Relic of the Fractal")]
    Fractal = 70047,

    #[serde(rename = "Relic of the Krait")]
    Krait = 69772,

    #[serde(rename = "Relic of the Nightmare")]
    Nightmare = 69626,

    #[serde(rename = "Relic of the Sunless")]
    Sunless = 69720,

    #[serde(rename = "Relic of the Thief")]
    Thief = 69914,

    #[serde(rename = "Relic of Vass")]
    Vass = 70697,
}
