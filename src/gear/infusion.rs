use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by an Infusion.
///
/// Missing most cosmetic infusions.
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
pub enum Infusion {
    #[serde(rename = "+1 Agony Infusion")]
    Agony1 = 22100,

    #[serde(rename = "+2 Agony Infusion")]
    Agony2 = 22101,

    #[serde(rename = "+3 Agony Infusion")]
    Agony3 = 22102,

    #[serde(rename = "+4 Agony Infusion")]
    Agony4 = 22103,

    #[serde(rename = "+5 Agony Infusion")]
    Agony5 = 22104,

    #[serde(rename = "+6 Agony Infusion")]
    Agony6 = 22105,

    #[serde(rename = "+7 Agony Infusion")]
    Agony7 = 22106,

    #[serde(rename = "+8 Agony Infusion")]
    Agony8 = 22107,

    #[serde(rename = "+9 Agony Infusion")]
    Agony9 = 22108,

    #[serde(rename = "+10 Agony Infusion")]
    Agony10 = 22109,

    #[serde(rename = "+11 Agony Infusion")]
    Agony11 = 22110,

    #[serde(rename = "+12 Agony Infusion")]
    Agony12 = 22111,

    #[serde(rename = "+13 Agony Infusion")]
    Agony13 = 22112,

    #[serde(rename = "+14 Agony Infusion")]
    Agony14 = 22113,

    #[serde(rename = "+15 Agony Infusion")]
    Agony15 = 22114,

    #[serde(rename = "+16 Agony Infusion")]
    Agony16 = 22115,

    #[serde(rename = "+17 Agony Infusion")]
    Agony17 = 22116,

    #[serde(rename = "+18 Agony Infusion")]
    Agony18 = 22117,

    #[serde(rename = "+19 Agony Infusion")]
    Agony19 = 22118,

    #[serde(rename = "+20 Agony Infusion")]
    Agony20 = 22119,

    #[serde(rename = "+21 Agony Infusion")]
    Agony21 = 22120,

    #[serde(rename = "+22 Agony Infusion")]
    Agony22 = 22121,

    #[serde(rename = "+23 Agony Infusion")]
    Agony23 = 22122,

    #[serde(rename = "+24 Agony Infusion")]
    Agony24 = 22123,

    #[serde(rename = "+25 Agony Infusion")]
    Agony25 = 22124,

    #[serde(rename = "+26 Agony Infusion")]
    Agony26 = 22125,

    #[serde(rename = "+27 Agony Infusion")]
    Agony27 = 22126,

    #[serde(rename = "+28 Agony Infusion")]
    Agony28 = 22127,

    #[serde(rename = "+29 Agony Infusion")]
    Agony29 = 22128,

    #[serde(rename = "+30 Agony Infusion")]
    Agony30 = 22129,

    #[serde(rename = "Abyssal Infusion (Concentration)")]
    AbyssalConcentration = 61488,

    #[serde(rename = "Abyssal Infusion (Condition Damage)")]
    AbyssalConditionDamage = 61540,

    #[serde(rename = "Abyssal Infusion (Expertise)")]
    AbyssalExpertise = 61334,

    #[serde(rename = "Abyssal Infusion (Healing)")]
    AbyssalHealing = 61545,

    #[serde(rename = "Abyssal Infusion (Power)")]
    AbyssalPower = 61262,

    #[serde(rename = "Abyssal Infusion (Precision)")]
    AbyssalPrecision = 61558,

    #[serde(rename = "Abyssal Infusion (Toughness)")]
    AbyssalToughness = 61413,

    #[serde(rename = "Abyssal Infusion (Vitality)")]
    AbyssalVitality = 61300,

    #[serde(rename = "Arcane Flow Infusion (Concentration)")]
    ArcaneFlowConcentration = 71713,

    #[serde(rename = "Arcane Flow Infusion (Condition Damage)")]
    ArcaneFlowConditionDamage = 71761,

    #[serde(rename = "Arcane Flow Infusion (Expertise)")]
    ArcaneFlowExpertise = 71411,

    #[serde(rename = "Arcane Flow Infusion (Healing)")]
    ArcaneFlowHealing = 71638,

    #[serde(rename = "Arcane Flow Infusion (Power)")]
    ArcaneFlowPower = 71492,

    #[serde(rename = "Arcane Flow Infusion (Precision)")]
    ArcaneFlowPrecision = 71760,

    #[serde(rename = "Arcane Flow Infusion (Toughness)")]
    ArcaneFlowToughness = 71441,

    #[serde(rename = "Arcane Flow Infusion (Vitality)")]
    ArcaneFlowVitality = 71590,

    #[serde(rename = "Bloodstone Infusion (Concentration)")]
    BloodstoneConcentration = 75500,

    #[serde(rename = "Bloodstone Infusion (Condition Damage)")]
    BloodstoneConditionDamage = 75521,

    #[serde(rename = "Bloodstone Infusion (Expertise)")]
    BloodstoneExpertise = 75480,

    #[serde(rename = "Bloodstone Infusion (Healing)")]
    BloodstoneHealing = 75768,

    #[serde(rename = "Bloodstone Infusion (Power)")]
    BloodstonePower = 75503,

    #[serde(rename = "Bloodstone Infusion (Precision)")]
    BloodstonePrecision = 75720,

    #[serde(rename = "Bloodstone Infusion (Toughness)")]
    BloodstoneToughness = 76024,

    #[serde(rename = "Bloodstone Infusion (Vitality)")]
    BloodstoneVitality = 75621,

    #[serde(rename = "Celestial Infusion (Blue) (Concentration)")]
    CelestialBlueConcentration = 48090,

    #[serde(rename = "Celestial Infusion (Blue) (Condition Damage)")]
    CelestialBlueConditionDamage = 39793,

    #[serde(rename = "Celestial Infusion (Blue) (Expertise)")]
    CelestialBlueExpertise = 47745,

    #[serde(rename = "Celestial Infusion (Blue) (Healing)")]
    CelestialBlueHealing = 39243,

    #[serde(rename = "Celestial Infusion (Blue) (Power)")]
    CelestialBluePower = 39326,

    #[serde(rename = "Celestial Infusion (Blue) (Precision)")]
    CelestialBluePrecision = 39881,

    #[serde(rename = "Celestial Infusion (Blue) (Toughness)")]
    CelestialBlueToughness = 39625,

    #[serde(rename = "Celestial Infusion (Blue) (Vitality)")]
    CelestialBlueVitality = 39780,

    #[serde(rename = "Celestial Infusion (Red) (Concentration)")]
    CelestialRedConcentration = 47979,

    #[serde(rename = "Celestial Infusion (Red) (Condition Damage)")]
    CelestialRedConditionDamage = 38959,

    #[serde(rename = "Celestial Infusion (Red) (Expertise)")]
    CelestialRedExpertise = 48252,

    #[serde(rename = "Celestial Infusion (Red) (Healing)")]
    CelestialRedHealing = 39169,

    #[serde(rename = "Celestial Infusion (Red) (Power)")]
    CelestialRedPower = 39548,

    #[serde(rename = "Celestial Infusion (Red) (Precision)")]
    CelestialRedPrecision = 39631,

    #[serde(rename = "Celestial Infusion (Red) (Toughness)")]
    CelestialRedToughness = 39431,

    #[serde(rename = "Celestial Infusion (Red) (Vitality)")]
    CelestialRedVitality = 39812,

    #[serde(rename = "Chak Infusion (Concentration)")]
    ChakConcentration = 47946,

    #[serde(rename = "Chak Infusion (Condition Damage)")]
    ChakConditionDamage = 39852,

    #[serde(rename = "Chak Infusion (Expertise)")]
    ChakExpertise = 48467,

    #[serde(rename = "Chak Infusion (Healing)")]
    ChakHealing = 39851,

    #[serde(rename = "Chak Infusion (Power)")]
    ChakPower = 39749,

    #[serde(rename = "Chak Infusion (Precision)")]
    ChakPrecision = 39155,

    #[serde(rename = "Chak Infusion (Toughness)")]
    ChakToughness = 39189,

    #[serde(rename = "Chak Infusion (Vitality)")]
    ChakVitality = 38921,

    #[serde(rename = "Chatoyant Infusion (Precision)")]
    ChatoyantPrecision = 76053,

    #[serde(rename = "Clockwork Infusion (Concentration)")]
    ClockworkConcentration = 68598,

    #[serde(rename = "Clockwork Infusion (Condition Damage)")]
    ClockworkConditionDamage = 68583,

    #[serde(rename = "Clockwork Infusion (Expertise)")]
    ClockworkExpertise = 68582,

    #[serde(rename = "Clockwork Infusion (Healing)")]
    ClockworkHealing = 68590,

    #[serde(rename = "Clockwork Infusion (Power)")]
    ClockworkPower = 68629,

    #[serde(rename = "Clockwork Infusion (Precision)")]
    ClockworkPrecision = 68633,

    #[serde(rename = "Clockwork Infusion (Toughness)")]
    ClockworkToughness = 68609,

    #[serde(rename = "Clockwork Infusion (Vitality)")]
    ClockworkVitality = 68635,

    #[serde(rename = "Concentration WvW Infusion (Concentration)")]
    ConcentrationWvWConcentration = 49710,

    #[serde(rename = "Crystal Infusion of Condition Damage (Condition Damage)")]
    CrystalofConditionDamageConditionDamage = 52967,

    #[serde(rename = "Crystal Infusion of Power (Power)")]
    CrystalofPowerPower = 51724,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Concentration)")]
    DeldrimorStoneskinConcentration = 62009,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Condition Damage)")]
    DeldrimorStoneskinConditionDamage = 61950,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Expertise)")]
    DeldrimorStoneskinExpertise = 62003,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Healing)")]
    DeldrimorStoneskinHealing = 62040,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Power)")]
    DeldrimorStoneskinPower = 62018,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Precision)")]
    DeldrimorStoneskinPrecision = 62000,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Toughness)")]
    DeldrimorStoneskinToughness = 61999,

    #[serde(rename = "Deldrimor Stoneskin Infusion (Vitality)")]
    DeldrimorStoneskinVitality = 62029,

    #[serde(rename = "Echo of the Dragonvoid (Concentration)")]
    EchoofDragonvoidConcentration = 67908,

    #[serde(rename = "Echo of the Dragonvoid (Condition Damage)")]
    EchoofDragonvoidConditionDamage = 67883,

    #[serde(rename = "Echo of the Dragonvoid (Expertise)")]
    EchoofDragonvoidExpertise = 67911,

    #[serde(rename = "Echo of the Dragonvoid (Healing)")]
    EchoofDragonvoidHealing = 67912,

    #[serde(rename = "Echo of the Dragonvoid (Power)")]
    EchoofDragonvoidPower = 67914,

    #[serde(rename = "Echo of the Dragonvoid (Precision)")]
    EchoofDragonvoidPrecision = 67879,

    #[serde(rename = "Echo of the Dragonvoid (Toughness)")]
    EchoofDragonvoidToughness = 67917,

    #[serde(rename = "Echo of the Dragonvoid (Vitality)")]
    EchoofDragonvoidVitality = 67938,

    #[serde(rename = "Ember Infusion (Concentration)")]
    EmberConcentration = 53236,

    #[serde(rename = "Ember Infusion (Condition Damage)")]
    EmberConditionDamage = 53240,

    #[serde(rename = "Ember Infusion (Expertise)")]
    EmberExpertise = 53244,

    #[serde(rename = "Ember Infusion (Healing)")]
    EmberHealing = 53225,

    #[serde(rename = "Ember Infusion (Power)")]
    EmberPower = 53230,

    #[serde(rename = "Ember Infusion (Precision)")]
    EmberPrecision = 53212,

    #[serde(rename = "Ember Infusion (Toughness)")]
    EmberToughness = 53229,

    #[serde(rename = "Ember Infusion (Vitality)")]
    EmberVitality = 53208,

    #[serde(rename = "Expertise WvW Infusion (Expertise)")]
    ExpertiseWvWExpertise = 49557,

    #[serde(rename = "Festive Confetti Infusion (Concentration)")]
    FestiveConfettiConcentration = 48496,

    #[serde(rename = "Festive Confetti Infusion (Condition Damage)")]
    FestiveConfettiConditionDamage = 46699,

    #[serde(rename = "Festive Confetti Infusion (Expertise)")]
    FestiveConfettiExpertise = 48449,

    #[serde(rename = "Festive Confetti Infusion (Healing)")]
    FestiveConfettiHealing = 46506,

    #[serde(rename = "Festive Confetti Infusion (Power)")]
    FestiveConfettiPower = 46591,

    #[serde(rename = "Festive Confetti Infusion (Precision)")]
    FestiveConfettiPrecision = 46479,

    #[serde(rename = "Festive Confetti Infusion (Toughness)")]
    FestiveConfettiToughness = 46491,

    #[serde(rename = "Festive Confetti Infusion (Vitality)")]
    FestiveConfettiVitality = 46659,

    #[serde(rename = "Forest Wisp Infusion (Power)")]
    ForestWispPower = 76036,

    #[serde(rename = "Frost Legion Infusion (Concentration)")]
    FrostLegionConcentration = 60920,

    #[serde(rename = "Frost Legion Infusion (Condition Damage)")]
    FrostLegionConditionDamage = 60928,

    #[serde(rename = "Frost Legion Infusion (Expertise)")]
    FrostLegionExpertise = 61129,

    #[serde(rename = "Frost Legion Infusion (Healing)")]
    FrostLegionHealing = 61008,

    #[serde(rename = "Frost Legion Infusion (Power)")]
    FrostLegionPower = 60745,

    #[serde(rename = "Frost Legion Infusion (Precision)")]
    FrostLegionPrecision = 61037,

    #[serde(rename = "Frost Legion Infusion (Toughness)")]
    FrostLegionToughness = 61034,

    #[serde(rename = "Frost Legion Infusion (Vitality)")]
    FrostLegionVitality = 60675,

    #[serde(rename = "Ghostly Infusion (Concentration)")]
    GhostlyConcentration = 47969,

    #[serde(rename = "Ghostly Infusion (Condition Damage)")]
    GhostlyConditionDamage = 34118,

    #[serde(rename = "Ghostly Infusion (Expertise)")]
    GhostlyExpertise = 48617,

    #[serde(rename = "Ghostly Infusion (Healing)")]
    GhostlyHealing = 34155,

    #[serde(rename = "Ghostly Infusion (Power)")]
    GhostlyPower = 34111,

    #[serde(rename = "Ghostly Infusion (Precision)")]
    GhostlyPrecision = 34109,

    #[serde(rename = "Ghostly Infusion (Toughness)")]
    GhostlyToughness = 34121,

    #[serde(rename = "Ghostly Infusion (Vitality)")]
    GhostlyVitality = 34133,

    #[serde(rename = "Healing +5 Agony Infusion")]
    Healing5Agony = 15738,

    #[serde(rename = "Healing +7 Agony Infusion")]
    Healing7Agony = 17064,

    #[serde(rename = "Healing +9 Agony Infusion")]
    Healing9Agony = 26287,

    #[serde(rename = "Healing Infusion")]
    Healing = 16986,

    #[serde(rename = "Healing WvW Infusion")]
    HealingWvW = 19062,

    #[serde(rename = "Heart of the Khan-Ur (Concentration)")]
    HeartofKhanUrConcentration = 57946,

    #[serde(rename = "Heart of the Khan-Ur (Condition Damage)")]
    HeartofKhanUrConditionDamage = 57483,

    #[serde(rename = "Heart of the Khan-Ur (Expertise)")]
    HeartofKhanUrExpertise = 57586,

    #[serde(rename = "Heart of the Khan-Ur (Healing)")]
    HeartofKhanUrHealing = 57850,

    #[serde(rename = "Heart of the Khan-Ur (Power)")]
    HeartofKhanUrPower = 57592,

    #[serde(rename = "Heart of the Khan-Ur (Precision)")]
    HeartofKhanUrPrecision = 57778,

    #[serde(rename = "Heart of the Khan-Ur (Toughness)")]
    HeartofKhanUrToughness = 58065,

    #[serde(rename = "Heart of the Khan-Ur (Vitality)")]
    HeartofKhanUrVitality = 57871,

    #[serde(rename = "Heat Core Infusion (Power)")]
    HeatCorePower = 74433,

    #[serde(rename = "Imperial Everbloom (Concentration)")]
    ImperialEverbloomConcentration = 68054,

    #[serde(rename = "Imperial Everbloom (Condition Damage)")]
    ImperialEverbloomConditionDamage = 68024,

    #[serde(rename = "Imperial Everbloom (Expertise)")]
    ImperialEverbloomExpertise = 67987,

    #[serde(rename = "Imperial Everbloom (Healing)")]
    ImperialEverbloomHealing = 68037,

    #[serde(rename = "Imperial Everbloom (Power)")]
    ImperialEverbloomPower = 67985,

    #[serde(rename = "Imperial Everbloom (Precision)")]
    ImperialEverbloomPrecision = 68012,

    #[serde(rename = "Imperial Everbloom (Toughness)")]
    ImperialEverbloomToughness = 68011,

    #[serde(rename = "Imperial Everbloom (Vitality)")]
    ImperialEverbloomVitality = 68010,

    #[serde(rename = "Jormag Left Eye Infusion (Concentration)")]
    JormagLeftEyeConcentration = 62436,

    #[serde(rename = "Jormag Left Eye Infusion (Condition Damage)")]
    JormagLeftEyeConditionDamage = 62188,

    #[serde(rename = "Jormag Left Eye Infusion (Expertise)")]
    JormagLeftEyeExpertise = 62355,

    #[serde(rename = "Jormag Left Eye Infusion (Healing)")]
    JormagLeftEyeHealing = 62362,

    #[serde(rename = "Jormag Left Eye Infusion (Power)")]
    JormagLeftEyePower = 62214,

    #[serde(rename = "Jormag Left Eye Infusion (Precision)")]
    JormagLeftEyePrecision = 62269,

    #[serde(rename = "Jormag Left Eye Infusion (Toughness)")]
    JormagLeftEyeToughness = 62180,

    #[serde(rename = "Jormag Left Eye Infusion (Vitality)")]
    JormagLeftEyeVitality = 62264,

    #[serde(rename = "Jormag Right Eye Infusion (Concentration)")]
    JormagRightEyeConcentration = 62278,

    #[serde(rename = "Jormag Right Eye Infusion (Condition Damage)")]
    JormagRightEyeConditionDamage = 62263,

    #[serde(rename = "Jormag Right Eye Infusion (Expertise)")]
    JormagRightEyeExpertise = 62312,

    #[serde(rename = "Jormag Right Eye Infusion (Healing)")]
    JormagRightEyeHealing = 62276,

    #[serde(rename = "Jormag Right Eye Infusion (Power)")]
    JormagRightEyePower = 62315,

    #[serde(rename = "Jormag Right Eye Infusion (Precision)")]
    JormagRightEyePrecision = 62439,

    #[serde(rename = "Jormag Right Eye Infusion (Toughness)")]
    JormagRightEyeToughness = 62282,

    #[serde(rename = "Jormag Right Eye Infusion (Vitality)")]
    JormagRightEyeVitality = 62161,

    #[serde(rename = "Jotun Infusion (Concentration)")]
    JotunConcentration = 71474,

    #[serde(rename = "Jotun Infusion (Condition Damage)")]
    JotunConditionDamage = 71542,

    #[serde(rename = "Jotun Infusion (Expertise)")]
    JotunExpertise = 71663,

    #[serde(rename = "Jotun Infusion (Healing)")]
    JotunHealing = 71564,

    #[serde(rename = "Jotun Infusion (Power)")]
    JotunPower = 71618,

    #[serde(rename = "Jotun Infusion (Precision)")]
    JotunPrecision = 71479,

    #[serde(rename = "Jotun Infusion (Toughness)")]
    JotunToughness = 71418,

    #[serde(rename = "Jotun Infusion (Vitality)")]
    JotunVitality = 71730,

    #[serde(rename = "Liquid Aurillium Infusion (Concentration)")]
    LiquidAurilliumConcentration = 47822,

    #[serde(rename = "Liquid Aurillium Infusion (Condition Damage)")]
    LiquidAurilliumConditionDamage = 39537,

    #[serde(rename = "Liquid Aurillium Infusion (Expertise)")]
    LiquidAurilliumExpertise = 47843,

    #[serde(rename = "Liquid Aurillium Infusion (Healing)")]
    LiquidAurilliumHealing = 39603,

    #[serde(rename = "Liquid Aurillium Infusion (Power)")]
    LiquidAurilliumPower = 39725,

    #[serde(rename = "Liquid Aurillium Infusion (Precision)")]
    LiquidAurilliumPrecision = 39517,

    #[serde(rename = "Liquid Aurillium Infusion (Toughness)")]
    LiquidAurilliumToughness = 39563,

    #[serde(rename = "Liquid Aurillium Infusion (Vitality)")]
    LiquidAurilliumVitality = 39675,

    #[serde(rename = "Malign +5 Agony Infusion")]
    Malign5Agony = 15737,

    #[serde(rename = "Malign +7 Agony Infusion")]
    Malign7Agony = 17063,

    #[serde(rename = "Malign +9 Agony Infusion")]
    Malign9Agony = 26290,

    #[serde(rename = "Malign Infusion")]
    Malign = 16987,

    #[serde(rename = "Malign WvW Infusion")]
    MalignWvW = 19061,

    #[serde(rename = "Mighty +5 Agony Infusion")]
    Mighty5Agony = 15735,

    #[serde(rename = "Mighty +7 Agony Infusion")]
    Mighty7Agony = 17065,

    #[serde(rename = "Mighty +9 Agony Infusion")]
    Mighty9Agony = 26291,

    #[serde(rename = "Mighty Infusion")]
    Mighty = 16985,

    #[serde(rename = "Mighty WvW Infusion")]
    MightyWvW = 19063,

    #[serde(rename = "Mistwalker Infusion (Concentration)")]
    MistwalkerConcentration = 69110,

    #[serde(rename = "Mistwalker Infusion (Condition Damage)")]
    MistwalkerConditionDamage = 69143,

    #[serde(rename = "Mistwalker Infusion (Expertise)")]
    MistwalkerExpertise = 69136,

    #[serde(rename = "Mistwalker Infusion (Healing)")]
    MistwalkerHealing = 69115,

    #[serde(rename = "Mistwalker Infusion (Power)")]
    MistwalkerPower = 69133,

    #[serde(rename = "Mistwalker Infusion (Precision)")]
    MistwalkerPrecision = 69113,

    #[serde(rename = "Mistwalker Infusion (Toughness)")]
    MistwalkerToughness = 69120,

    #[serde(rename = "Mistwalker Infusion (Vitality)")]
    MistwalkerVitality = 69142,

    #[serde(rename = "Mote of Darkness (Concentration)")]
    MoteofDarknessConcentration = 69417,

    #[serde(rename = "Mote of Darkness (Condition Damage)")]
    MoteofDarknessConditionDamage = 69393,

    #[serde(rename = "Mote of Darkness (Expertise)")]
    MoteofDarknessExpertise = 69335,

    #[serde(rename = "Mote of Darkness (Healing)")]
    MoteofDarknessHealing = 69367,

    #[serde(rename = "Mote of Darkness (Power)")]
    MoteofDarknessPower = 69213,

    #[serde(rename = "Mote of Darkness (Precision)")]
    MoteofDarknessPrecision = 69188,

    #[serde(rename = "Mote of Darkness (Toughness)")]
    MoteofDarknessToughness = 69233,

    #[serde(rename = "Mote of Darkness (Vitality)")]
    MoteofDarknessVitality = 69255,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Concentration)")]
    MotosUnstableBaubleBlueConcentration = 48886,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Condition Damage)")]
    MotosUnstableBaubleBlueConditionDamage = 34592,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Expertise)")]
    MotosUnstableBaubleBlueExpertise = 48880,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Healing)")]
    MotosUnstableBaubleBlueHealing = 34597,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Power)")]
    MotosUnstableBaubleBluePower = 34578,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Precision)")]
    MotosUnstableBaubleBluePrecision = 34594,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Toughness)")]
    MotosUnstableBaubleBlueToughness = 34585,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Blue (Vitality)")]
    MotosUnstableBaubleBlueVitality = 34601,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Concentration)")]
    MotosUnstableBaubleRedConcentration = 48892,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Condition Damage)")]
    MotosUnstableBaubleRedConditionDamage = 34580,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Expertise)")]
    MotosUnstableBaubleRedExpertise = 48911,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Healing)")]
    MotosUnstableBaubleRedHealing = 34574,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Power)")]
    MotosUnstableBaubleRedPower = 34582,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Precision)")]
    MotosUnstableBaubleRedPrecision = 34577,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Toughness)")]
    MotosUnstableBaubleRedToughness = 34593,

    #[serde(rename = "Moto's Unstable Bauble Infusion: Red (Vitality)")]
    MotosUnstableBaubleRedVitality = 34583,

    #[serde(rename = "Mystic Infusion (Concentration)")]
    MysticConcentration = 55989,

    #[serde(rename = "Mystic Infusion (Condition Damage)")]
    MysticConditionDamage = 55992,

    #[serde(rename = "Mystic Infusion (Expertise)")]
    MysticExpertise = 55956,

    #[serde(rename = "Mystic Infusion (Healing)")]
    MysticHealing = 55984,

    #[serde(rename = "Mystic Infusion (Power)")]
    MysticPower = 55955,

    #[serde(rename = "Mystic Infusion (Precision)")]
    MysticPrecision = 56003,

    #[serde(rename = "Mystic Infusion (Toughness)")]
    MysticToughness = 55987,

    #[serde(rename = "Mystic Infusion (Vitality)")]
    MysticVitality = 55963,

    #[serde(rename = "Mystical +5 Agony Infusion")]
    Mystical5Agony = 47878,

    #[serde(rename = "Mystical +7 Agony Infusion")]
    Mystical7Agony = 47436,

    #[serde(rename = "Mystical +9 Agony Infusion")]
    Mystical9Agony = 48048,

    #[serde(rename = "Peerless Infusion (Concentration)")]
    PeerlessConcentration = 56290,

    #[serde(rename = "Peerless Infusion (Condition Damage)")]
    PeerlessConditionDamage = 56278,

    #[serde(rename = "Peerless Infusion (Expertise)")]
    PeerlessExpertise = 56425,

    #[serde(rename = "Peerless Infusion (Healing)")]
    PeerlessHealing = 56631,

    #[serde(rename = "Peerless Infusion (Power)")]
    PeerlessPower = 56066,

    #[serde(rename = "Peerless Infusion (Precision)")]
    PeerlessPrecision = 56547,

    #[serde(rename = "Peerless Infusion (Toughness)")]
    PeerlessToughness = 56177,

    #[serde(rename = "Peerless Infusion (Vitality)")]
    PeerlessVitality = 56628,

    #[serde(rename = "Phospholuminescent Infusion (Concentration)")]
    PhospholuminescentConcentration = 47514,

    #[serde(rename = "Phospholuminescent Infusion (Condition Damage)")]
    PhospholuminescentConditionDamage = 36816,

    #[serde(rename = "Phospholuminescent Infusion (Expertise)")]
    PhospholuminescentExpertise = 47422,

    #[serde(rename = "Phospholuminescent Infusion (Healing)")]
    PhospholuminescentHealing = 36789,

    #[serde(rename = "Phospholuminescent Infusion (Power)")]
    PhospholuminescentPower = 36818,

    #[serde(rename = "Phospholuminescent Infusion (Precision)")]
    PhospholuminescentPrecision = 36806,

    #[serde(rename = "Phospholuminescent Infusion (Toughness)")]
    PhospholuminescentToughness = 36836,

    #[serde(rename = "Phospholuminescent Infusion (Vitality)")]
    PhospholuminescentVitality = 36773,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Concentration)")]
    PolyluminescentUndulatingBlackConcentration = 47178,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Condition Damage)")]
    PolyluminescentUndulatingBlackConditionDamage = 39570,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Expertise)")]
    PolyluminescentUndulatingBlackExpertise = 47255,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Healing)")]
    PolyluminescentUndulatingBlackHealing = 39289,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Power)")]
    PolyluminescentUndulatingBlackPower = 39318,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Precision)")]
    PolyluminescentUndulatingBlackPrecision = 39796,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Toughness)")]
    PolyluminescentUndulatingBlackToughness = 39420,

    #[serde(rename = "Polyluminescent Undulating Infusion (Black) (Vitality)")]
    PolyluminescentUndulatingBlackVitality = 39722,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Concentration)")]
    PolyluminescentUndulatingGreenConcentration = 48634,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Condition Damage)")]
    PolyluminescentUndulatingGreenConditionDamage = 39168,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Expertise)")]
    PolyluminescentUndulatingGreenExpertise = 47551,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Healing)")]
    PolyluminescentUndulatingGreenHealing = 39123,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Power)")]
    PolyluminescentUndulatingGreenPower = 39802,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Precision)")]
    PolyluminescentUndulatingGreenPrecision = 39365,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Toughness)")]
    PolyluminescentUndulatingGreenToughness = 39370,

    #[serde(rename = "Polyluminescent Undulating Infusion (Green) (Vitality)")]
    PolyluminescentUndulatingGreenVitality = 39743,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Concentration)")]
    PolyluminescentUndulatingOrangeConcentration = 47786,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Condition Damage)")]
    PolyluminescentUndulatingOrangeConditionDamage = 38879,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Expertise)")]
    PolyluminescentUndulatingOrangeExpertise = 48337,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Healing)")]
    PolyluminescentUndulatingOrangeHealing = 39127,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Power)")]
    PolyluminescentUndulatingOrangePower = 39184,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Precision)")]
    PolyluminescentUndulatingOrangePrecision = 39772,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Toughness)")]
    PolyluminescentUndulatingOrangeToughness = 39334,

    #[serde(rename = "Polyluminescent Undulating Infusion (Orange) (Vitality)")]
    PolyluminescentUndulatingOrangeVitality = 38901,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Concentration)")]
    PolyluminescentUndulatingTealConcentration = 48644,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Condition Damage)")]
    PolyluminescentUndulatingTealConditionDamage = 39403,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Expertise)")]
    PolyluminescentUndulatingTealExpertise = 48538,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Healing)")]
    PolyluminescentUndulatingTealHealing = 39499,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Power)")]
    PolyluminescentUndulatingTealPower = 38862,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Precision)")]
    PolyluminescentUndulatingTealPrecision = 39283,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Toughness)")]
    PolyluminescentUndulatingTealToughness = 38930,

    #[serde(rename = "Polyluminescent Undulating Infusion (Teal) (Vitality)")]
    PolyluminescentUndulatingTealVitality = 39296,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Concentration)")]
    PolysaturatingReverberatingGrayConcentration = 53247,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Condition Damage)")]
    PolysaturatingReverberatingGrayConditionDamage = 53235,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Expertise)")]
    PolysaturatingReverberatingGrayExpertise = 53224,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Healing)")]
    PolysaturatingReverberatingGrayHealing = 53215,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Power)")]
    PolysaturatingReverberatingGrayPower = 53219,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Precision)")]
    PolysaturatingReverberatingGrayPrecision = 53243,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Toughness)")]
    PolysaturatingReverberatingGrayToughness = 53245,

    #[serde(rename = "Polysaturating Reverberating Infusion (Gray) (Vitality)")]
    PolysaturatingReverberatingGrayVitality = 53207,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Concentration)")]
    PolysaturatingReverberatingPurpleConcentration = 53246,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Condition Damage)")]
    PolysaturatingReverberatingPurpleConditionDamage = 53231,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Expertise)")]
    PolysaturatingReverberatingPurpleExpertise = 53238,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Healing)")]
    PolysaturatingReverberatingPurpleHealing = 53223,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Power)")]
    PolysaturatingReverberatingPurplePower = 53226,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Precision)")]
    PolysaturatingReverberatingPurplePrecision = 53221,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Toughness)")]
    PolysaturatingReverberatingPurpleToughness = 53237,

    #[serde(rename = "Polysaturating Reverberating Infusion (Purple) (Vitality)")]
    PolysaturatingReverberatingPurpleVitality = 53242,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Concentration)")]
    PolysaturatingReverberatingRedConcentration = 53211,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Condition Damage)")]
    PolysaturatingReverberatingRedConditionDamage = 53218,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Expertise)")]
    PolysaturatingReverberatingRedExpertise = 53239,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Healing)")]
    PolysaturatingReverberatingRedHealing = 53234,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Power)")]
    PolysaturatingReverberatingRedPower = 53220,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Precision)")]
    PolysaturatingReverberatingRedPrecision = 53217,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Toughness)")]
    PolysaturatingReverberatingRedToughness = 53214,

    #[serde(rename = "Polysaturating Reverberating Infusion (Red) (Vitality)")]
    PolysaturatingReverberatingRedVitality = 53206,

    #[serde(rename = "Possession Infusion (Concentration)")]
    PossessionConcentration = 70897,

    #[serde(rename = "Possession Infusion (Condition Damage)")]
    PossessionConditionDamage = 70455,

    #[serde(rename = "Possession Infusion (Expertise)")]
    PossessionExpertise = 71204,

    #[serde(rename = "Possession Infusion (Healing)")]
    PossessionHealing = 69883,

    #[serde(rename = "Possession Infusion (Power)")]
    PossessionPower = 71158,

    #[serde(rename = "Possession Infusion (Precision)")]
    PossessionPrecision = 71144,

    #[serde(rename = "Possession Infusion (Toughness)")]
    PossessionToughness = 69750,

    #[serde(rename = "Possession Infusion (Vitality)")]
    PossessionVitality = 70206,

    #[serde(rename = "Precise +5 Agony Infusion")]
    Precise5Agony = 15736,

    #[serde(rename = "Precise +7 Agony Infusion")]
    Precise7Agony = 17066,

    #[serde(rename = "Precise +9 Agony Infusion")]
    Precise9Agony = 26289,

    #[serde(rename = "Precise Infusion")]
    Precise = 16984,

    #[serde(rename = "Precise WvW Infusion")]
    PreciseWvW = 19064,

    #[serde(rename = "Primordus Left Eye Infusion (Concentration)")]
    PrimordusLeftEyeConcentration = 62353,

    #[serde(rename = "Primordus Left Eye Infusion (Condition Damage)")]
    PrimordusLeftEyeConditionDamage = 62387,

    #[serde(rename = "Primordus Left Eye Infusion (Expertise)")]
    PrimordusLeftEyeExpertise = 62411,

    #[serde(rename = "Primordus Left Eye Infusion (Healing)")]
    PrimordusLeftEyeHealing = 62318,

    #[serde(rename = "Primordus Left Eye Infusion (Power)")]
    PrimordusLeftEyePower = 62361,

    #[serde(rename = "Primordus Left Eye Infusion (Precision)")]
    PrimordusLeftEyePrecision = 62202,

    #[serde(rename = "Primordus Left Eye Infusion (Toughness)")]
    PrimordusLeftEyeToughness = 62438,

    #[serde(rename = "Primordus Left Eye Infusion (Vitality)")]
    PrimordusLeftEyeVitality = 62234,

    #[serde(rename = "Primordus Right Eye Infusion (Concentration)")]
    PrimordusRightEyeConcentration = 62333,

    #[serde(rename = "Primordus Right Eye Infusion (Condition Damage)")]
    PrimordusRightEyeConditionDamage = 62235,

    #[serde(rename = "Primordus Right Eye Infusion (Expertise)")]
    PrimordusRightEyeExpertise = 62232,

    #[serde(rename = "Primordus Right Eye Infusion (Healing)")]
    PrimordusRightEyeHealing = 62433,

    #[serde(rename = "Primordus Right Eye Infusion (Power)")]
    PrimordusRightEyePower = 62323,

    #[serde(rename = "Primordus Right Eye Infusion (Precision)")]
    PrimordusRightEyePrecision = 62203,

    #[serde(rename = "Primordus Right Eye Infusion (Toughness)")]
    PrimordusRightEyeToughness = 62225,

    #[serde(rename = "Primordus Right Eye Infusion (Vitality)")]
    PrimordusRightEyeVitality = 62210,

    #[serde(rename = "Queen Bee Infusion (Concentration)")]
    QueenBeeConcentration = 47271,

    #[serde(rename = "Queen Bee Infusion (Condition Damage)")]
    QueenBeeConditionDamage = 39613,

    #[serde(rename = "Queen Bee Infusion (Expertise)")]
    QueenBeeExpertise = 48325,

    #[serde(rename = "Queen Bee Infusion (Healing)")]
    QueenBeeHealing = 39197,

    #[serde(rename = "Queen Bee Infusion (Power)")]
    QueenBeePower = 39506,

    #[serde(rename = "Queen Bee Infusion (Precision)")]
    QueenBeePrecision = 39741,

    #[serde(rename = "Queen Bee Infusion (Toughness)")]
    QueenBeeToughness = 39006,

    #[serde(rename = "Queen Bee Infusion (Vitality)")]
    QueenBeeVitality = 39509,

    #[serde(rename = "Resilient +5 Agony Infusion")]
    Resilient5Agony = 15748,

    #[serde(rename = "Resilient +7 Agony Infusion")]
    Resilient7Agony = 17067,

    #[serde(rename = "Resilient +9 Agony Infusion")]
    Resilient9Agony = 26286,

    #[serde(rename = "Resilient Infusion")]
    Resilient = 16983,

    #[serde(rename = "Resilient WvW Infusion")]
    ResilientWvW = 19065,

    #[serde(rename = "Silent Symphony (Concentration)")]
    SilentSymphonyConcentration = 69473,

    #[serde(rename = "Silent Symphony (Condition Damage)")]
    SilentSymphonyConditionDamage = 69443,

    #[serde(rename = "Silent Symphony (Expertise)")]
    SilentSymphonyExpertise = 69454,

    #[serde(rename = "Silent Symphony (Healing)")]
    SilentSymphonyHealing = 69439,

    #[serde(rename = "Silent Symphony (Power)")]
    SilentSymphonyPower = 69477,

    #[serde(rename = "Silent Symphony (Precision)")]
    SilentSymphonyPrecision = 69465,

    #[serde(rename = "Silent Symphony (Toughness)")]
    SilentSymphonyToughness = 69450,

    #[serde(rename = "Silent Symphony (Vitality)")]
    SilentSymphonyVitality = 69459,

    #[serde(rename = "Snow Diamond Infusion (Concentration)")]
    SnowDiamondConcentration = 48936,

    #[serde(rename = "Snow Diamond Infusion (Condition Damage)")]
    SnowDiamondConditionDamage = 48906,

    #[serde(rename = "Snow Diamond Infusion (Expertise)")]
    SnowDiamondExpertise = 48885,

    #[serde(rename = "Snow Diamond Infusion (Healing)")]
    SnowDiamondHealing = 48920,

    #[serde(rename = "Snow Diamond Infusion (Power)")]
    SnowDiamondPower = 48938,

    #[serde(rename = "Snow Diamond Infusion (Precision)")]
    SnowDiamondPrecision = 48896,

    #[serde(rename = "Snow Diamond Infusion (Toughness)")]
    SnowDiamondToughness = 48929,

    #[serde(rename = "Snow Diamond Infusion (Vitality)")]
    SnowDiamondVitality = 48904,

    #[serde(rename = "Spiteful +5 Agony Infusion")]
    Spiteful5Agony = 48782,

    #[serde(rename = "Spiteful +7 Agony Infusion")]
    Spiteful7Agony = 48364,

    #[serde(rename = "Spiteful +9 Agony Infusion")]
    Spiteful9Agony = 48652,

    #[serde(rename = "Toy-Shell Infusion (Concentration)")]
    ToyShellConcentration = 53458,

    #[serde(rename = "Toy-Shell Infusion (Condition Damage)")]
    ToyShellConditionDamage = 53455,

    #[serde(rename = "Toy-Shell Infusion (Expertise)")]
    ToyShellExpertise = 53481,

    #[serde(rename = "Toy-Shell Infusion (Healing)")]
    ToyShellHealing = 53439,

    #[serde(rename = "Toy-Shell Infusion (Power)")]
    ToyShellPower = 53542,

    #[serde(rename = "Toy-Shell Infusion (Precision)")]
    ToyShellPrecision = 53535,

    #[serde(rename = "Toy-Shell Infusion (Toughness)")]
    ToyShellToughness = 53459,

    #[serde(rename = "Toy-Shell Infusion (Vitality)")]
    ToyShellVitality = 53440,

    #[serde(rename = "Vital +5 Agony Infusion")]
    Vital5Agony = 15747,

    #[serde(rename = "Vital +7 Agony Infusion")]
    Vital7Agony = 17068,

    #[serde(rename = "Vital +9 Agony Infusion")]
    Vital9Agony = 26288,

    #[serde(rename = "Vital Infusion")]
    Vital = 16982,

    #[serde(rename = "Vital WvW Infusion")]
    VitalWvW = 19066,

    #[serde(rename = "Winter's Heart Infusion (Concentration)")]
    WintersHeartConcentration = 47706,

    #[serde(rename = "Winter's Heart Infusion (Condition Damage)")]
    WintersHeartConditionDamage = 37590,

    #[serde(rename = "Winter's Heart Infusion (Expertise)")]
    WintersHeartExpertise = 48293,

    #[serde(rename = "Winter's Heart Infusion (Healing)")]
    WintersHeartHealing = 37586,

    #[serde(rename = "Winter's Heart Infusion (Power)")]
    WintersHeartPower = 37587,

    #[serde(rename = "Winter's Heart Infusion (Precision)")]
    WintersHeartPrecision = 37594,

    #[serde(rename = "Winter's Heart Infusion (Toughness)")]
    WintersHeartToughness = 37591,

    #[serde(rename = "Winter's Heart Infusion (Vitality)")]
    WintersHeartVitality = 37601,
}
