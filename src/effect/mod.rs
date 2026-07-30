mod info;

use self::info::*;
use crate::{Agent, Time};
use evtc_parse::{Event, EventKind, Log, Position, content::ContentInfo, effect::effect51};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Ground {
        time: i32,
        tracking_id: u32,
        effect: Option<EffectInfo>,
        source: Agent,
        duration: u32,
        location: Position,
        orientation: Position,
        scale: f32,
        moving_platform: u8,
    },
    Agent {
        time: i32,
        tracking_id: u32,
        effect: Option<EffectInfo>,
        source: Agent,
        target: Agent,
        duration: u32,
    },
    GroundEnd {
        time: i32,
        tracking_id: u32,
    },
    AgentEnd {
        time: i32,
        tracking_id: u32,
        agent: Agent,
    },
}

pub fn extract_effects<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a Event> + Clone,
) -> Vec<Effect> {
    let start = Time::log_start(log);
    let guids: HashMap<u32, EffectInfo> = log
        .events
        .iter()
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
                        source: Agent::from_agent(&event.source, log),
                        target: Agent::from_id(id, log),
                        duration,
                        tracking_id,
                    },
                    effect51::EffectLocation::Position(location) => Effect::Ground {
                        time,
                        effect,
                        source: Agent::from_agent(&event.source, log),
                        duration,
                        tracking_id,
                        location,
                        orientation: event.orientation.as_position(),
                        scale: 0.0,
                        moving_platform: event.moving_platform,
                    },
                })
            }

            EventKind::EffectGroundCreate(event) => Some(Effect::Ground {
                time: start.relative(event.time),
                effect: guids.get(&event.effect_id).cloned(),
                source: Agent::from_agent(&event.source, log),
                duration: event.duration,
                tracking_id: event.tracking_id,
                location: event.location,
                orientation: event.orientation,
                scale: event.scale,
                moving_platform: event.moving_platform,
            }),

            EventKind::EffectGroundRemove(event) => Some(Effect::GroundEnd {
                time: start.relative(event.time),
                tracking_id: event.tracking_id,
            }),

            EventKind::EffectAgentCreate(event) => Some(Effect::Agent {
                time: start.relative(event.time),
                effect: guids.get(&event.effect_id).cloned(),
                source: Agent::from_agent(&event.source, log),
                target: Agent::from_agent(&event.target, log),
                duration: event.duration,
                tracking_id: event.tracking_id,
            }),

            EventKind::EffectAgentRemove(event) => Some(Effect::AgentEnd {
                time: start.relative(event.time),
                agent: Agent::from_agent(&event.agent, log),
                tracking_id: event.tracking_id,
            }),

            _ => None,
        })
        .collect()
}
