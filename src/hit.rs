use crate::{Agent, Skill};
use evtc_parse::{strike::StrikeEvent, Event, Log, Strike, TryExtract};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub time: i32,
    pub target: Agent,
    pub kind: Strike,
    pub damage: i32,
}

impl Hit {
    pub fn try_from_event(log: &Log, event: &Event, time: i32) -> Option<Self> {
        StrikeEvent::try_extract(event).map(|event| Self {
            time,
            target: Agent::from_log(event.dst.id, log),
            kind: event.strike,
            damage: event.total_damage,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithSkill {
    #[serde(flatten)]
    pub hit: Hit,
    pub skill: Skill,
}
