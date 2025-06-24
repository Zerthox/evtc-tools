use crate::{Agent, SkillIdName};
use evtc_parse::{strike::StrikeEvent, Log, Strike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub time: i32,
    pub target: Agent,
    pub kind: Strike,
    pub damage: i32,
}

impl Hit {
    pub fn from_strike(log: &Log, event: &StrikeEvent, time: i32) -> Self {
        Self {
            time,
            target: Agent::from_log(event.target.id, log),
            kind: event.strike,
            damage: event.total_damage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithSkill {
    #[serde(flatten)]
    pub hit: Hit,
    pub skill: SkillIdName,
}
