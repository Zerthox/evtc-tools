use crate::{skill_name, Agent};
use arcdps_parse::{Activation, BuffRemove, CombatEvent, Log, StateChange, Strike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub time: u64,
    pub id: u32,
    pub name: Option<String>,
    pub agent: Agent,
    pub hits: Vec<Hit>,
}

impl Cast {
    pub fn new(id: u32, name: Option<impl Into<String>>, agent: Agent, time: u64) -> Self {
        Self {
            id,
            name: name.map(Into::into),
            agent,
            time,
            hits: Vec::new(),
        }
    }

    pub fn add_hit(&mut self, kind: Strike, damage: i32, time: u64) {
        self.hits.push(Hit {
            kind,
            damage,
            time: time.saturating_sub(self.time),
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub kind: Strike,
    pub damage: i32,
    pub time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithoutCast {
    pub id: u32,
    pub agent: u64,
    pub time: u64,
}

// TODO: include quickness gained/lost
#[allow(unused)]
const QUICKNESS: u32 = 1187;

pub fn extract_casts(
    log: &Log,
    skill_filter: u32,
    agent_filter: Option<u64>,
) -> (Vec<Cast>, Vec<HitWithoutCast>) {
    let mut casts = HashMap::<_, Vec<_>>::new();
    let mut hits_without_cast = Vec::new();

    for event in &log.events {
        let id = event.skill_id;
        if id == skill_filter && agent_filter.map(|id| event.src_agent == id).unwrap_or(true) {
            match event {
                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::Start,
                    ..
                } => {
                    // activation start
                    let agent_id = event.src_agent;
                    let agent = Agent::from_log(agent_id, log);
                    let cast = Cast::new(id, skill_name(&log.skills, id), agent, event.time);
                    casts.entry(agent_id).or_default().push(cast);
                }

                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::None,
                    is_buff_remove: BuffRemove::None,
                    buff: 0,
                    ..
                } => {
                    // direct damage
                    let agent_id = event.src_agent;
                    match casts.get_mut(&agent_id).and_then(|casts| casts.last_mut()) {
                        Some(cast) => cast.add_hit(event.result.into(), event.value, event.time),
                        None => hits_without_cast.push(HitWithoutCast {
                            id,
                            agent: agent_id,
                            time: event.time,
                        }),
                    }
                }

                _ => {}
            }
        }
    }

    let mut casts: Vec<_> = casts.into_iter().flat_map(|(_, cast)| cast).collect();
    casts.sort_by_key(|cast| cast.time);

    (casts, hits_without_cast)
}
