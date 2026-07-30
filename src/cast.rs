use crate::{Agent, Hit, SkillIdName, Time, util::to_tick_rounded};
use evtc_parse::{Event, EventKind, Log, animation::AnimationProgress, event::AnimationStop};
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
    pub duration_scaled: Option<i32>,
    pub duration_unscaled: Option<i32>,
    pub hits: Vec<CastHit>,
}

impl Cast {
    pub fn new(skill: SkillIdName, agent: Agent, time: i32) -> Self {
        Self {
            time,
            skill,
            agent,
            end: AnimationProgress::None,
            duration_scaled: None,
            duration_unscaled: None,
            hits: Vec::new(),
        }
    }

    pub fn add_end(&mut self, event: &AnimationStop) {
        self.end = event.progress;
        self.duration_scaled = Some(event.duration_scaled);
        self.duration_unscaled = Some(event.duration_unscaled);
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
    pub skill: SkillIdName,
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
            let skill = SkillIdName::from_log(log, skill_id);
            let time = start.relative(event.time);

            match event.clone().into_kind() {
                EventKind::AnimationStart(event) => {
                    let agent = Agent::from_agent(&event.agent, log);
                    let cast = Cast::new(skill, agent, time);
                    casts
                        .entry((event.agent.id, skill_id))
                        .or_default()
                        .push(cast);
                }
                EventKind::AnimationStop(event) => {
                    if let Some(cast) = casts
                        .get_mut(&(event.agent.id, skill_id))
                        .and_then(|casts| casts.last_mut())
                    {
                        cast.add_end(&event);
                    }
                }
                EventKind::Combat(event) => {
                    if let Some(hit) = Hit::try_from_event(log, &event, time) {
                        match casts
                            .get_mut(&(event.source.id, skill_id))
                            .and_then(|casts| casts.last_mut())
                        {
                            Some(cast) => cast.add_hit(hit),
                            None => hits_without_cast.push(StandaloneHit {
                                hit,
                                skill,
                                agent: Agent::from_agent(&event.source, log),
                            }),
                        }
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
