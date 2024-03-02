use evtc_parse::event as evtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub recharge: f32,
    pub range0: f32,
    pub range1: f32,
    pub tooltip_time: f32,
    pub timings: Vec<SkillTiming>,
}

impl SkillInfo {
    pub fn new(
        info: evtc::SkillInfo,
        timings: impl IntoIterator<Item = evtc::SkillTiming>,
    ) -> Self {
        Self {
            recharge: info.recharge,
            range0: info.range0,
            range1: info.range1,
            tooltip_time: info.tooltip_time,
            timings: timings.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTiming {
    pub action: u64,
    pub millisecond: u64,
}

impl From<evtc::SkillTiming> for SkillTiming {
    fn from(event: evtc::SkillTiming) -> Self {
        Self {
            action: event.action,
            millisecond: event.millisecond,
        }
    }
}
