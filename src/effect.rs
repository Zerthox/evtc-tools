use crate::{Agent, Time};
use evtc_parse::{
    effect::{ContentLocal, Effect as RawEffect, EffectGUID, EffectLocation},
    Event, Log, Position,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub time: i32,
    pub effect_id: u32,
    #[serde(flatten)]
    pub info: Option<EffectInfo>,
    pub owner: Option<Agent>,
    pub moving_platform: u8,
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

impl From<EffectGUID> for EffectInfo {
    fn from(event: EffectGUID) -> Self {
        Self {
            guid: event.guid_string(),
            content_local: event.content_local,
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
        .filter_map(|event| event.try_extract::<EffectGUID>())
        .map(|event| (event.effect_id, event.into()))
        .collect();

    events
        .filter_map(|event| event.try_extract::<RawEffect>())
        .map(|effect| {
            let info = guids.get(&effect.effect_id).cloned();
            Effect {
                time: start.relative(effect.time),
                effect_id: effect.effect_id,
                info,
                owner: if effect.owner != 0 {
                    Some(Agent::from_log(effect.owner, log))
                } else {
                    None
                },
                moving_platform: effect.moving_platform,
                location: effect.location,
                duration: effect.duration,
                tracking_id: effect.tracking_id,
                orientation: effect.orientation.into(),
            }
        })
        .collect()
}
