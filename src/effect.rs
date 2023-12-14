use crate::{Agent, Time};
use evtc_parse::{self as evtc, effect::ContentLocal, Event, Log, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub time: i32,
    pub effect_id: u32,
    #[serde(flatten)]
    pub info: Option<EffectInfo>,
    pub owner: Option<Agent>,
    pub location: EffectLocation,
    pub duration: u32,
    pub tracking_id: u32,
    pub orientation: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectInfo {
    pub guid: String,
    pub content_local: Option<ContentLocal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectLocation {
    Agent(Agent),
    Position(Position),
}

impl EffectLocation {
    pub fn from_location(location: evtc::effect::EffectLocation, log: &Log) -> Self {
        match location {
            evtc::effect::EffectLocation::Agent(id) => Self::Agent(Agent::from_log(id, log)),
            evtc::effect::EffectLocation::Position(pos) => Self::Position(pos),
        }
    }
}

pub fn extract_effects<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a Event> + Clone,
) -> Vec<Effect> {
    let start = Time::log_start(log);
    let guids: HashMap<_, _> = events
        .clone()
        .filter_map(|event| event.try_extract::<evtc::effect::EffectGUID>())
        .map(|event| {
            (
                event.effect_id,
                EffectInfo {
                    guid: event.guid_string(),
                    content_local: event.content_local,
                },
            )
        })
        .collect();

    events
        .filter_map(|event| {
            event.try_extract::<evtc::effect::Effect>().map(|effect| {
                let info = guids.get(&effect.effect_id).cloned();
                Effect {
                    time: start.relative(event.time),
                    effect_id: effect.effect_id,
                    info,
                    owner: if effect.owner != 0 {
                        Some(Agent::from_log(effect.owner, log))
                    } else {
                        None
                    },
                    location: EffectLocation::from_location(effect.location, log),
                    duration: effect.duration,
                    tracking_id: effect.tracking_id,
                    orientation: effect.orientation.into(),
                }
            })
        })
        .collect()
}
