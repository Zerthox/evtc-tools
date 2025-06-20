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

    #[serde(rename = "Sigil of Agility")]
    Agility = 33690,

    #[serde(rename = "Sigil of Agony")]
    Agony = 9313,

    #[serde(rename = "Sigil of Air")]
    Air = 9448,

    #[serde(rename = "Sigil of Battle")]
    Battle = 9424,

    #[serde(rename = "Sigil of Benevolence")]
    Benevolence = 9401,

    #[serde(rename = "Sigil of Blight")]
    Blight = 25875,

    #[serde(rename = "Sigil of Blood")]
    Blood = 9458,

    #[serde(rename = "Sigil of Bloodlust")]
    Bloodlust = 9297,

    #[serde(rename = "Sigil of Bounty")]
    Bounty = 38663,

    #[serde(rename = "Sigil of Bursting")]
    Bursting = 20474,

    #[serde(rename = "Sigil of Celerity")]
    Celerity = 24208,

    #[serde(rename = "Sigil of Centaur Slaying")]
    CentaurSlaying = 9334,

    #[serde(rename = "Sigil of Chilling")]
    Chilling = 9315,

    #[serde(rename = "Sigil of Cleansing")]
    Cleansing = 25519,

    #[serde(rename = "Sigil of Concentration")]
    Concentration = 33913,

    #[serde(rename = "Sigil of Corruption")]
    Corruption = 9377,

    #[serde(rename = "Sigil of Cruelty")]
    Cruelty = 25522,

    #[serde(rename = "Sigil of Debility")]
    Debility = 9319,

    #[serde(rename = "Sigil of Demon Slaying")]
    DemonSlaying = 9353,

    #[serde(rename = "Sigil of Demons")]
    Demons = 9395,

    #[serde(rename = "Sigil of Destroyer Slaying")]
    DestroyerSlaying = 9343,

    #[serde(rename = "Sigil of Doom")]
    Doom = 9442,

    #[serde(rename = "Sigil of Draining")]
    Draining = 33022,

    #[serde(rename = "Sigil of Dreams")]
    Dreams = 9370,

    #[serde(rename = "Sigil of Earth")]
    Earth = 9452,

    #[serde(rename = "Sigil of Elemental Slaying")]
    ElementalSlaying = 9352,

    #[serde(rename = "Sigil of Energy")]
    Energy = 9438,

    #[serde(rename = "Sigil of Fire")]
    Fire = 9444,

    #[serde(rename = "Sigil of Force")]
    Force = 9322,

    #[serde(rename = "Sigil of Frailty")]
    Frailty = 9456,

    #[serde(rename = "Sigil of Frenzy")]
    Frenzy = 41722,

    #[serde(rename = "Sigil of Generosity")]
    Generosity = 16509,

    #[serde(rename = "Sigil of Geomancy")]
    Geomancy = 9436,

    #[serde(rename = "Sigil of Ghost Slaying")]
    GhostSlaying = 9701,

    #[serde(rename = "Sigil of Grawl Slaying")]
    GrawlSlaying = 9335,

    #[serde(rename = "Sigil of Hobbling")]
    Hobbling = 9316,

    #[serde(rename = "Sigil of Hologram Slaying")]
    HologramSlaying = 56840,

    #[serde(rename = "Sigil of Hydromancy")]
    Hydromancy = 9430,

    #[serde(rename = "Sigil of Ice")]
    Ice = 9450,

    #[serde(rename = "Sigil of Icebrood Slaying")]
    IcebroodSlaying = 9340,

    #[serde(rename = "Sigil of Impact")]
    Impact = 9718,

    #[serde(rename = "Sigil of Incapacitation")]
    Incapacitation = 25520,

    #[serde(rename = "Sigil of Justice")]
    Justice = 9365,

    #[serde(rename = "Sigil of Karka Slaying")]
    KarkaSlaying = 15770,

    #[serde(rename = "Sigil of Leeching")]
    Leeching = 9420,

    #[serde(rename = "Sigil of Life")]
    Life = 9389,

    #[serde(rename = "Sigil of Luck")]
    Luck = 9410,

    #[serde(rename = "Sigil of Mad Scientists")]
    MadScientists = 9359,

    #[serde(rename = "Sigil of Malice")]
    Malice = 20468,

    #[serde(rename = "Sigil of Mischief")]
    Mischief = 26274,

    #[serde(rename = "Sigil of Momentum")]
    Momentum = 22141,

    #[serde(rename = "Sigil of Nullification")]
    Nullification = 9290,

    #[serde(rename = "Sigil of Ogre Slaying")]
    OgreSlaying = 9344,

    #[serde(rename = "Sigil of Paralyzation")]
    Paralyzation = 9328,

    #[serde(rename = "Sigil of Perception")]
    Perception = 9382,

    #[serde(rename = "Sigil of Peril")]
    Peril = 9318,

    #[serde(rename = "Sigil of Purity")]
    Purity = 9289,

    #[serde(rename = "Sigil of Rage")]
    Rage = 9295,

    #[serde(rename = "Sigil of Rending")]
    Rending = 32673,

    #[serde(rename = "Sigil of Renewal")]
    Renewal = 20471,

    #[serde(rename = "Sigil of Restoration")]
    Restoration = 9413,

    #[serde(rename = "Sigil of Ruthlessness")]
    Ruthlessness = 32496,

    #[serde(rename = "Sigil of Serpent Slaying")]
    SerpentSlaying = 9347,

    #[serde(rename = "Sigil of Severance")]
    Severance = 42433,

    #[serde(rename = "Sigil of Smoldering")]
    Smoldering = 9314,

    #[serde(rename = "Sigil of Smothering")]
    Smothering = 9362,

    #[serde(rename = "Sigil of Sorrow")]
    Sorrow = 9373,

    #[serde(rename = "Sigil of Speed")]
    Speed = 9404,

    #[serde(rename = "Sigil of Stamina")]
    Stamina = 9414,

    #[serde(rename = "Sigil of Strength")]
    Strength = 9454,

    #[serde(rename = "Sigil of the Night")]
    Night = 15268,

    #[serde(rename = "Sigil of the Stars")]
    Stars = 47393,

    #[serde(rename = "Sigil of Torment")]
    Torment = 21824,

    #[serde(rename = "Sigil of Transference")]
    Transference = 32819,

    #[serde(rename = "Sigil of Undead Slaying")]
    UndeadSlaying = 9331,

    #[serde(rename = "Sigil of Venom")]
    Venom = 9317,

    #[serde(rename = "Sigil of Vision")]
    Vision = 53276,

    #[serde(rename = "Sigil of Water")]
    Water = 9446,

    #[serde(rename = "Sigil of Wrath")]
    Wrath = 9356,
}
