use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by an Infusion.
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
enum Infusion {
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
}
