mod info;

use self::info::*;
use crate::{Agent, Time};
use evtc_parse::{content::ContentInfo, effect::effect51, Event, EventKind, Log, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Ground {
        time: i32,
        effect: Option<EffectInfo>,
        owner: Agent,
        duration: u32,
        tracking_id: u32,
        location: Position,
        orientation: Position,
        moving_platform: u8,
    },
    Agent {
        time: i32,
        effect: Option<EffectInfo>,
        agent: Agent,
        duration: u32,
        tracking_id: u32,
    },
}

pub fn extract_effects<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a Event> + Clone,
) -> Vec<Effect> {
    let start = Time::log_start(log);
    let guids: HashMap<u32, EffectInfo> = events
        .clone()
        .filter_map(|event| EffectInfo::new(event.try_extract::<ContentInfo>()?))
        .map(|info| (info.content_id, info))
        .collect();

    events
        .filter_map(|event| match event.clone().into_kind() {
            EventKind::Effect45(_) => todo!("effect 45"),

            EventKind::Effect51(event) => {
                let effect = guids.get(&event.effect_id).cloned();
                let time = start.relative(event.time);
                let duration = event.duration;
                let tracking_id = event.tracking_id;

                Some(match event.location {
                    effect51::EffectLocation::Agent(id) => Effect::Agent {
                        time,
                        effect,
                        agent: Agent::from_log(id, log),
                        duration,
                        tracking_id,
                    },
                    effect51::EffectLocation::Position(location) => Effect::Ground {
                        time,
                        effect,
                        owner: Agent::from_log(event.owner.id, log),
                        duration,
                        tracking_id,
                        location,
                        orientation: event.orientation.as_position(),
                        moving_platform: event.moving_platform,
                    },
                })
            }

            EventKind::EffectGroundCreate(event) => Some(Effect::Ground {
                time: start.relative(event.time),
                effect: guids.get(&event.effect_id).cloned(),
                owner: Agent::from_log(event.owner.id, log),
                duration: event.duration,
                tracking_id: event.tracking_id,
                location: event.location,
                orientation: event.orientation,
                moving_platform: event.moving_platform,
            }),

            EventKind::EffectAgentCreate(event) => Some(Effect::Agent {
                time: start.relative(event.time),
                effect: guids.get(&event.effect_id).cloned(),
                agent: Agent::from_log(event.agent.id, log),
                duration: event.duration,
                tracking_id: event.tracking_id,
            }),

            _ => None,
        })
        .collect()
}
