use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by a Superior Sigil.
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
pub enum Sigil {
    #[serde(rename = "Sigil of Absorption")]
    Absorption = 33756,

    #[serde(rename = "Sigil of Accuracy")]
    Accuracy = 9325,

    #[serde(rename = "Sigil of Accuracy (Stats)")]
    AccuracyStats = 24151,

    #[serde(rename = "Sigil of Agony")]
    Agony = 9313,

    #[serde(rename = "Sigil of Agony (Stats)")]
    AgonyStats = 24158,

    #[serde(rename = "Sigil of Air")]
    Air = 9448,

    #[serde(rename = "Sigil of Blight")]
    Blight = 25875,

    #[serde(rename = "Sigil of Bursting")]
    Bursting = 20474,

    #[serde(rename = "Sigil of Celerity")]
    Celerity = 24208,

    #[serde(rename = "Sigil of Concentration")]
    Concentration = 33913,

    #[serde(rename = "Sigil of Demons")]
    Demons = 9395,

    #[serde(rename = "Sigil of Doom")]
    Doom = 9442,

    #[serde(rename = "Sigil of Earth")]
    Earth = 9452,

    #[serde(rename = "Sigil of Energy")]
    Energy = 9438,

    #[serde(rename = "Sigil of Fire")]
    Fire = 9444,

    #[serde(rename = "Sigil of Force")]
    Force = 9322,

    #[serde(rename = "Sigil of Force (Stats)")]
    ForceStats = 24148,

    #[serde(rename = "Sigil of Frailty")]
    Frailty = 9456,

    #[serde(rename = "Sigil of Frenzy")]
    Frenzy = 41722,

    #[serde(rename = "Sigil of Geomancy")]
    Geomancy = 9436,

    #[serde(rename = "Sigil of Hydromancy")]
    Hydromancy = 9430,

    #[serde(rename = "Sigil of Impact")]
    Impact = 9718,

    #[serde(rename = "Sigil of Malice")]
    Malice = 20468,

    #[serde(rename = "Sigil of Mischief")]
    Mischief = 26274,

    #[serde(rename = "Sigil of Night")]
    Night = 15268,

    #[serde(rename = "Sigil of Nullification")]
    Nullification = 9290,

    #[serde(rename = "Sigil of Paralyzation")]
    Paralyzation = 9328,

    #[serde(rename = "Sigil of Rage")]
    Rage = 9295,

    #[serde(rename = "Sigil of Renewal")]
    Renewal = 20471,

    #[serde(rename = "Sigil of Serpent Slaying")]
    SerpentSlaying = 9347,

    #[serde(rename = "Sigil of Severance")]
    Severance = 42433,

    #[serde(rename = "Sigil of Smoldering")]
    Smoldering = 9314,

    #[serde(rename = "Sigil of Smoldering (Stats)")]
    SmolderingStats = 24168,

    #[serde(rename = "Sigil of Torment")]
    Torment = 21824,

    #[serde(rename = "Sigil of Transference")]
    Transference = 32819,

    #[serde(rename = "Sigil of Venom")]
    Venom = 9317,

    #[serde(rename = "Sigil of Venom (Stats)")]
    VenomStats = 24157,

    #[serde(rename = "Sigil of Water")]
    Water = 9446,
}
