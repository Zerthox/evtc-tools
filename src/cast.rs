use crate::{log_start, util::to_tick_rounded, Agent, Hit, Skill};
use arcdps_parse::{Activation, BuffRemove, CombatEvent, Log, StateChange};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Casts {
    pub casts: Vec<Cast>,
    pub hits_without_cast: Vec<StandaloneHit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub time: u64,
    pub skill: Skill,
    pub agent: Agent,
    pub hits: Vec<CastHit>,
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

    pub fn add_hit(&mut self, mut hit: Hit) {
        hit.time -= self.time;
        self.hits.push(CastHit {
            tick: to_tick_rounded(hit.time),
            hit,
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastHit {
    pub tick: u64,

    #[serde(flatten)]
    pub hit: Hit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandaloneHit {
    pub skill: Skill,
    pub agent: Agent,

    #[serde(flatten)]
    pub hit: Hit,
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
                    ..
                } => {
                    let hit = Hit::try_from_event(log, event).unwrap();

                    match casts
                        .get_mut(&(src_agent, skill_id))
                        .and_then(|casts| casts.last_mut())
                    {
                        Some(cast) => cast.add_hit(hit),
                        None => hits_without_cast.push(StandaloneHit {
                            hit,
                            skill,
                            agent: Agent::from_log(src_agent, log),
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
