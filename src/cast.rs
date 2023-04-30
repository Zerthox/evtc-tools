use crate::{log_start, util::to_tick_rounded, Agent};
use arcdps_parse::{Activation, BuffRemove, CombatEvent, Log, StateChange, Strike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Casts {
    pub casts: Vec<Cast>,
    pub hits_without_cast: Vec<HitWithoutCast>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub time: u64,
    pub skill: Skill,
    pub agent: Agent,
    pub hits: Vec<HitWithCast>,
}

impl Cast {
    pub fn new(skill: Skill, agent: Agent, time: u64) -> Self {
        Self {
            skill,
            agent,
            time,
            hits: Vec::new(),
        }
    }

    pub fn add_hit(&mut self, hit: Hit) {
        let Hit {
            time,
            kind,
            damage,
            target,
        } = hit;
        let time = time.saturating_sub(self.time);
        self.hits.push(HitWithCast {
            time,
            tick: to_tick_rounded(time),
            kind,
            damage,
            target,
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub time: u64,
    pub target: Agent,
    pub kind: Strike,
    pub damage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithCast {
    pub time: u64,
    pub tick: u64,
    pub target: Agent,
    pub kind: Strike,
    pub damage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitWithoutCast {
    pub time: u64,
    pub skill: Skill,
    pub agent: Agent,
    pub target: Agent,
    pub kind: Strike,
    pub damage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: Option<String>,
}

impl Skill {
    pub fn from_log(log: &Log, id: u32) -> Self {
        Self {
            id,
            name: log.skill_name(id).map(Into::into),
        }
    }
}

pub fn extract_casts<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a CombatEvent>,
    skill: Option<u32>,
) -> Casts {
    let start = log_start(log);
    let mut casts = HashMap::<_, Vec<_>>::new();
    let mut hits_without_cast = Vec::new();

    for event in events {
        let skill_id = event.skill_id;
        if skill.map(|skill| skill_id == skill).unwrap_or(true) {
            let skill = Skill::from_log(log, skill_id);
            let time = event.time - start;

            match *event {
                // activation start
                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::Start,

                    src_agent,
                    ..
                } => {
                    let agent = Agent::from_log(src_agent, log);
                    let cast = Cast::new(skill, agent, time);
                    casts.entry((src_agent, skill_id)).or_default().push(cast);
                }

                // direct damage
                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::None,
                    is_buff_remove: BuffRemove::None,
                    buff: 0,
                    src_agent,
                    dst_agent,
                    result: kind,
                    value: damage,
                    ..
                } => {
                    let kind = kind.into();
                    let target = Agent::from_log(dst_agent, log);
                    match casts
                        .get_mut(&(src_agent, skill_id))
                        .and_then(|casts| casts.last_mut())
                    {
                        Some(cast) => cast.add_hit(Hit {
                            time,
                            target,
                            kind,
                            damage,
                        }),
                        None => hits_without_cast.push(HitWithoutCast {
                            time,
                            skill,
                            agent: Agent::from_log(src_agent, log),
                            target,
                            kind,
                            damage,
                        }),
                    }
                }

                _ => {}
            }
        }
    }

    let mut casts: Vec<_> = casts.into_iter().flat_map(|(_, cast)| cast).collect();
    casts.sort_by_key(|cast| cast.time);

    Casts {
        casts,
        hits_without_cast,
    }
}
