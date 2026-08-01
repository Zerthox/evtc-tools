mod hit;
mod missile;

pub use self::{hit::*, missile::*};

use crate::{Agent, Hit, SkillIdName, Time};
use evtc_parse::{
    Event, EventKind, Log,
    animation::AnimationProgress,
    event::{AnimationStop, MissileLaunch},
};
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
    pub skill: SkillIdName,
    pub agent: Agent,
    pub end: AnimationProgress,
    pub duration: Option<i32>,
    pub missiles: Vec<CastMissile>,
    pub hits: Vec<CastHit>,
}

impl Cast {
    pub fn new(skill: SkillIdName, agent: Agent, time: i32) -> Self {
        Self {
            time,
            skill,
            agent,
            end: AnimationProgress::None,
            duration: None,
            missiles: Vec::new(),
            hits: Vec::new(),
        }
    }

    pub fn add_end(&mut self, event: &AnimationStop) {
        self.end = event.progress;
        self.duration = Some(event.duration_scaled);
    }

    pub fn add_missile(&mut self, time: i32, event: &MissileLaunch, log: &Log) {
        self.missiles
            .push(CastMissile::new(time - self.time, event, log));
    }

    pub fn add_hit(&mut self, mut hit: Hit) {
        hit.time -= self.time;
        self.hits.push(CastHit::new(hit));
    }
}

struct CastData {
    pub map: HashMap<(u64, u32), Vec<Cast>>,
    pub standalone: Vec<StandaloneHit>,
}

impl CastData {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            standalone: Vec::new(),
        }
    }

    pub fn get_or_create(&mut self, agent_id: u64, skill_id: u32) -> &mut Vec<Cast> {
        self.map.entry((agent_id, skill_id)).or_default()
    }

    pub fn get_last(&mut self, agent_id: u64, skill_id: u32) -> Option<&mut Cast> {
        self.map
            .get_mut(&(agent_id, skill_id))
            .and_then(|casts| casts.last_mut())
    }

    pub fn add_standalone(&mut self, hit: StandaloneHit) {
        self.standalone.push(hit)
    }

    pub fn finalize(self) -> Casts {
        let mut casts: Vec<_> = self.map.into_iter().flat_map(|(_, cast)| cast).collect();
        casts.sort_by_key(|cast| cast.time);

        Casts {
            casts,
            hits_without_cast: self.standalone,
        }
    }
}

pub fn extract_casts<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a Event>,
    skill_filter: Option<u32>,
) -> Casts {
    let start = Time::log_start(log);
    let mut casts = CastData::new();

    for event in events {
        let skill_id = event.skill_id;
        if skill_filter.is_none_or(|skill| skill_id == skill) {
            let skill = SkillIdName::from_log(log, skill_id);
            let time = start.relative(event.time);

            match event.clone().into_kind() {
                EventKind::AnimationStart(event) => {
                    let agent = Agent::from_agent(&event.agent, log);
                    let cast = Cast::new(skill, agent, time);
                    casts.get_or_create(cast.agent.id, skill_id).push(cast);
                }
                EventKind::AnimationStop(event) => {
                    if let Some(cast) = casts.get_last(event.agent.id, skill_id) {
                        cast.add_end(&event);
                    }
                }
                EventKind::MissileLaunch(event) => {
                    if let Some(cast) = casts.get_last(event.source.id, skill_id) {
                        cast.add_missile(time, &event, log)
                    }
                }
                EventKind::Combat(event) => {
                    if let Some(hit) = Hit::try_from_event(log, &event, time) {
                        match casts.get_last(event.source.id, skill_id) {
                            Some(cast) => cast.add_hit(hit),
                            None => casts.add_standalone(StandaloneHit::new(
                                skill,
                                Agent::from_agent(&event.source, log),
                                hit,
                            )),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    casts.finalize()
}
