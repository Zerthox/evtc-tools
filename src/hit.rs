use crate::{Agent, SkillIdName};
use evtc_parse::{CombatEvent, CombatResult, Log};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub time: i32,
    pub target: Agent,
    pub result: CombatResult,
    pub damage: i32,
}

impl Hit {
    pub fn try_from_event(log: &Log, event: &CombatEvent, time: i32) -> Option<Self> {
        event.result.is_strike_damage().then(|| Self {
            time,
            target: Agent::from_agent(&event.target, log),
            result: event.result,
            damage: event.total_strike_damage,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithSkill {
    #[serde(flatten)]
    pub hit: Hit,
    pub skill: SkillIdName,
}
