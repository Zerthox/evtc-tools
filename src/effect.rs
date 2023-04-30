use crate::{log_start, Agent};
use arcdps_parse::{CombatEvent, ContentLocal, EffectDuration, Log, Position, StateChange};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, mem::transmute};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub time: u64,
    pub id: u32,
    #[serde(flatten)]
    pub info: Option<EffectInfo>,
    pub owner: Agent,
    pub location: EffectLocation,
    pub duration: EffectDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectInfo {
    pub guid: String,
    pub content_local: Option<ContentLocal>,
}

impl EffectInfo {
    pub fn new(guid: u128, content_local: Option<ContentLocal>) -> Self {
        Self {
            guid: format!("{:0>32X}", guid),
            content_local,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectLocation {
    Agent(Agent),
    Coordinates {
        position: Position,
        orientation: Position,
    },
}

pub fn extract_effects<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a CombatEvent> + Clone,
) -> Vec<Effect> {
    let start = log_start(log);
    let guids: HashMap<_, _> = events
        .clone()
        .filter(|event| event.is_statechange == StateChange::IdToGUID)
        .map(|event| {
            (
                event.skill_id,
                EffectInfo::new(
                    u128::from_be_bytes(unsafe { transmute([event.src_agent, event.dst_agent]) }),
                    event.overstack_value.try_into().ok(),
                ),
            )
        })
        .collect();

    events
        .filter_map(|event| {
            event.effect().map(|effect| {
                let info = guids.get(&effect.effect_id).cloned();
                Effect {
                    time: event.time - start,
                    id: effect.effect_id,
                    owner: Agent::from_log(effect.owner, log),
                    location: if effect.agent_location != 0 {
                        EffectLocation::Agent(Agent::from_log(effect.agent_location, log))
                    } else {
                        EffectLocation::Coordinates {
                            position: effect.location,
                            orientation: effect.orientation,
                        }
                    },
                    duration: effect.duration,
                    info,
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_format() {
        let guid1: u128 = 0;
        let guid2: u128 = 0x0123456789ABCDEF0123456789ABCDEF;
        let info1 = EffectInfo::new(guid1, None);
        let info2 = EffectInfo::new(guid2, None);

        assert_eq!(info1.guid, "00000000000000000000000000000000");
        assert_eq!(info2.guid, "0123456789ABCDEF0123456789ABCDEF");
    }
}
