use crate::{Agent, Skill};
use arcdps_parse::{Activation, BuffRemove, CombatEvent, Log, StateChange, Strike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub time: u64,
    pub target: Agent,
    pub kind: Option<Strike>,
    pub damage: i32,
}

impl Hit {
    pub fn try_from_event(log: &Log, event: &CombatEvent) -> Option<Self> {
        match *event {
            CombatEvent {
                time,
                is_statechange: StateChange::None,
                is_activation: Activation::None,
                is_buff_remove: BuffRemove::None,
                buff: 0,
                dst_agent,
                result: kind,
                value: damage,
                ..
            } => Some(Self {
                time,
                target: Agent::from_log(dst_agent, log),
                kind: kind.try_into().ok(),
                damage,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithSkill {
    pub skill: Skill,

    #[serde(flatten)]
    pub hit: Hit,
}
