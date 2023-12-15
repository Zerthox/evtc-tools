use crate::{util::to_tick_rounded, Agent, Hit, Skill, Time};
use evtc_parse::{Activation, Event, EventKind, Log};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Casts {
    pub casts: Vec<Cast>,
    pub hits_without_cast: Vec<StandaloneHit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub time: i32,
    pub skill: Skill,
    pub agent: Agent,
    pub kind: Option<Activation>,
    pub duration: Option<i32>,
    pub hits: Vec<CastHit>,
}

impl Cast {
    pub fn new(skill: Skill, agent: Agent, time: i32) -> Self {
        Self {
            time,
            skill,
            agent,
            kind: None,
            duration: None,
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
    pub tick: i32,

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
    events: impl Iterator<Item = &'a Event>,
    skill: Option<u32>,
) -> Casts {
    let start = Time::log_start(log);
    let mut casts = HashMap::<_, Vec<_>>::new();
    let mut hits_without_cast = Vec::new();

    for event in events {
        let skill_id = event.skill_id;
        if skill.map(|skill| skill_id == skill).unwrap_or(true) {
            let skill = Skill::from_log(log, skill_id);
            let time = start.relative(event.time);

            match event.clone().into_kind() {
                EventKind::Activation(event) => {
                    let agent_id = event.agent.id;
                    match event.activation {
                        Activation::Start => {
                            let agent = Agent::from_log(agent_id, log);
                            let cast = Cast::new(skill, agent, time);
                            casts.entry((agent_id, skill_id)).or_default().push(cast);
                        }
                        kind @ (Activation::Reset
                        | Activation::CancelCancel
                        | Activation::CancelFire) => {
                            if let Some(cast) = casts
                                .get_mut(&(agent_id, skill_id))
                                .and_then(|casts| casts.last_mut())
                            {
                                cast.kind = Some(kind);
                                cast.duration = Some(event.duration);
                            }
                        }
                        _ => {}
                    }
                }

                EventKind::Strike(event) => {
                    let hit = Hit::from_strike(log, &event, time);
                    let agent = event.src.id;
                    match casts
                        .get_mut(&(agent, skill_id))
                        .and_then(|casts| casts.last_mut())
                    {
                        Some(cast) => cast.add_hit(hit),
                        None => hits_without_cast.push(StandaloneHit {
                            hit,
                            skill,
                            agent: Agent::from_log(agent, log),
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
