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

    #[serde(rename = "Healing +5 Agony Infusion")]
    Healing5 = 15738,

    #[serde(rename = "Malign +5 Agony Infusion")]
    Malign5 = 15737,

    #[serde(rename = "Mighty +5 Agony Infusion")]
    Mighty5 = 15735,

    #[serde(rename = "Mystical +5 Agony Infusion")]
    Mystical5 = 47878,

    #[serde(rename = "Precise +5 Agony Infusion")]
    Precise5 = 15736,

    #[serde(rename = "Resilient +5 Agony Infusion")]
    Resilient5 = 15748,

    #[serde(rename = "Spiteful +5 Agony Infusion")]
    Spiteful5 = 48782,

    #[serde(rename = "Vital +5 Agony Infusion")]
    Vital5 = 15747,

    #[serde(rename = "Healing +7 Agony Infusion")]
    Healing7 = 17064,

    #[serde(rename = "Malign +7 Agony Infusion")]
    Malign7 = 17063,

    #[serde(rename = "Mighty +7 Agony Infusion")]
    Mighty7 = 17065,

    #[serde(rename = "Mystical +7 Agony Infusion")]
    Mystical7 = 47436,

    #[serde(rename = "Precise +7 Agony Infusion")]
    Precise7 = 17066,

    #[serde(rename = "Resilient +7 Agony Infusion")]
    Resilient7 = 17067,

    #[serde(rename = "Spiteful +7 Agony Infusion")]
    Spiteful7 = 48364,

    #[serde(rename = "Vital +7 Agony Infusion")]
    Vital7 = 17068,

    #[serde(rename = "Healing +9 Agony Infusion")]
    Healing9 = 26287,

    #[serde(rename = "Malign +9 Agony Infusion")]
    Malign9 = 26290,

    #[serde(rename = "Mighty +9 Agony Infusion")]
    Mighty9 = 26291,

    #[serde(rename = "Mystical +9 Agony Infusion")]
    Mystical9 = 47514,

    #[serde(rename = "Precise +9 Agony Infusion")]
    Precise9 = 26289,

    #[serde(rename = "Resilient +9 Agony Infusion")]
    Resilient9 = 26286,

    #[serde(rename = "Spiteful +9 Agony Infusion")]
    Spiteful9 = 47422,

    #[serde(rename = "Vital +9 Agony Infusion")]
    Vital9 = 26288,

    #[serde(rename = "Mighty Arcane Flow Infusion")]
    MightyArcaneFlow = 71492,
}
